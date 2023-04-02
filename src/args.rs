use std::path::PathBuf;

use clap::{Parser};

pub const PROGRAM_VERSION: &str = "1.0";
#[derive(Parser, Debug)]
#[command(author = "DynamicCake", version = PROGRAM_VERSION, about = "Big cope seethe mald", long_about = None)]
pub struct Args {

    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_delimiter = ' ')]
    pub messages: Option<Vec<String>>,

    #[arg(short, long)]
    pub start: Option<String>,

    #[arg(short, long)]
    pub length: usize

}