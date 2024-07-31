use std::collections::HashMap;
use clap::Parser;
use cli::Cli;
use reqwest_eventsource::{Event, EventSource};
use futures::stream::StreamExt;
use anyhow::{Error, Result};
mod cli;

async fn post(cli: &Cli) -> Result<()>{
    let mut data = HashMap::new();
    data.insert("branch".to_string(), cli.branch.clone());
    data.insert("commit_id".to_string(), cli.commit_id.clone());
    let url = format!("http://{}:{}/test", cli.backend_address, cli.port);

    // post 请求要创建client
    let response = reqwest::Client::new()
        .post(url)
        .json(&data);
    let mut es = EventSource::new(response).expect("Failed to create EventSource");
    let mut failed = false;
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => {
                if message.data == "Exit Code: 1"{
                    failed = true;
                    es.close();
                }
                println!("{}", message.data);
            }
            Err(err) => {
                if let reqwest_eventsource::Error::StreamEnded = err {
                    println!("{}", err);
                }else{
                    println!("Sse Error: {}", err);
                    failed = true;
                }
                es.close();
            }
        }
    }

    if failed {
        return Err(Error::msg(""));
    }

    // 发起post请求并返回
    // Ok(client.post(url).headers(headers).send().await?.json::<HashMap<String, Vec<u8>>>().await?)
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    post(&cli).await.expect("Test Failed");
}
