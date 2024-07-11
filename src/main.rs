use std::collections::HashMap;
use clap::Parser;
use cli::Cli;
use reqwest_eventsource::{Event, EventSource};
use tokio::io::{self, AsyncWriteExt};
use futures::stream::StreamExt;
mod cli;

async fn post(cli: &Cli) -> Result<HashMap<String, Vec<u8>>, reqwest::Error>{
    let mut data = HashMap::new();
    data.insert("branch".to_string(), cli.branch.clone());
    data.insert("commit_id".to_string(), cli.commit_id.clone());
    let url = format!("http://{}:{}/test", cli.backend_address, cli.port);

    // post 请求要创建client
    let response = reqwest::Client::new()
        .post(url)
        .json(&data);
    let mut es = EventSource::new(response).expect("Failed to create EventSource");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                println!("{}", message.data); // not sure how to send this messages to user
            }
            Err(err) => {
                println!("Sse Error: {}", err);
                es.close();
            }
        }
    }

    // 发起post请求并返回
    // Ok(client.post(url).headers(headers).send().await?.json::<HashMap<String, Vec<u8>>>().await?)
    Ok(HashMap::new())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    post(&cli).await.unwrap();
}
