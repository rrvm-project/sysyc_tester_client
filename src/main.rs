use std::collections::HashMap;
use clap::Parser;
use cli::Cli;
use reqwest::header::HeaderMap;
use tokio::io::{self, AsyncWriteExt};


mod cli;

async fn post(cli: &Cli) -> Result<HashMap<String, Vec<u8>>, reqwest::Error>{
    let url = format!("http://{}:{}/test/{}/{}", cli.backend_address, cli.port, cli.branch, cli.commit_id);

    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 发起post请求并返回
    Ok(client.post(url).headers(headers).send().await?.json::<HashMap<String, Vec<u8>>>().await?)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();


    match post(&cli).await {
        Ok(res) => {
            io::stdout().write_all(&res["func_output"]).await.unwrap();
            io::stdout().write_all(&res["perf_output"]).await.unwrap();
        },
        Err(e) => println!("{:#?}", e),
    }
}
