use std::collections::HashMap;
use clap::Parser;
use cli::Cli;
use reqwest::header::HeaderMap;
use serde_json::value::Value;


mod cli;

async fn post(cli: &Cli) -> Result<HashMap<String, Value>, reqwest::Error>{
    let url = format!("http://{}:{}/test/{}/{}", cli.backend_address, cli.port, cli.branch, cli.commit_id);

    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 组装要提交的数据
    let mut data: HashMap<String, String> = HashMap::new();
    // data.insert("user", "tangjz");
    // data.insert("password", "dev-tang.com");

    // 发起post请求并返回
    Ok(client.post(url).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();


    match post(&cli).await {
        Ok(res) => println!("{:#?}", res),
        Err(e) => println!("{:#?}", e),
    }
}
