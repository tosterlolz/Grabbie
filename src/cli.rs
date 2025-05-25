use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    pub urls: Vec<String>,

    #[arg(short = 'P', long)]
    pub post: bool,

    #[arg(short = 'd', long)]
    pub data_file: Option<String>,
}
