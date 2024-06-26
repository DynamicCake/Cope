use std::path::PathBuf;

use clap::Parser;

pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Parser, Debug)]
#[command(author = "DynamicCake", version = PROGRAM_VERSION, about = "Big cope seethe mald", long_about = None)]
pub struct Args {
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[arg(short, long)]
    pub combo: Option<String>,

    #[arg(short, long)]
    pub start: Option<String>,

    #[arg(short, long)]
    pub length: usize,
}

