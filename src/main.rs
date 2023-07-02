use std::{
    fs::File,
    io::{Error, Read},
    path::PathBuf, cmp,
};

use args::Args;
use clap::Parser;
use parse::Config;
use rand::seq::SliceRandom;

use crate::args::PROGRAM_VERSION;

mod args;
mod parse;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let path = args.config.unwrap_or(PathBuf::from("config.json"));

    let Ok(mut config_file) = File::open(&path) else {
        println!("Cannot open config file {}", &path.to_string_lossy());
        return Ok(());
    };

    let mut contents = String::new();

    if config_file.read_to_string(&mut contents).is_err() {
        println!("Could not read file {}", &path.to_string_lossy());
        return Ok(());
    }

    let mut config: Config = match serde_json::from_str(&contents) {
        Ok(it) => it, 
        Err(err) => {
            println!("Could not parse bad JSON: {}", err);
            return Ok(());
        } 
    };

    if config.version != PROGRAM_VERSION {
        println!("Warning: Config version: {} does not match program version: {}", config.version, PROGRAM_VERSION);
    }

    let message_list = args.messages.unwrap_or(vec![config.default_list]);


    let mut options = Vec::new();

    for messages in message_list {
        let found_list = match config.message_list.get_mut(&messages) {
            Some(it) => it,
            None => {
                println!("Warning: pack {} is not found", messages);
                continue;
            }
        };
        
        options.append(found_list);
    }

    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng);

    let start_name = args.start.unwrap_or(config.default_start);
    let mut out: Vec<String> = match config.start_list.get(&start_name) {
        Some(it) => it.clone(), 
        None => {
            println!("Error: cannot get default start");
            return Ok(());
        }
    };
    out.append(&mut options);
    let new_len = cmp::min(args.length, out.len());
    out.resize_with(new_len, || {
        // This should never be reached
        println!("Warning: There was a problem resizing the array size");
        String::new()
    }); 

    let out = out.join(&config.separator);    

    println!("{:?}", out);

    Ok(())
}
