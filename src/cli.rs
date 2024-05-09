use clap::Parser;


#[derive(Parser, Clone)]
#[clap(author, version, about="")]
pub struct Cli {
    #[clap(short='a', long, value_parser)]
    pub backend_address: String,
    #[clap(short, long)]
    pub port: String,
    #[clap(short, long)]
    pub branch: String,
    #[clap(short, long)]
    pub commit_id: String,
}