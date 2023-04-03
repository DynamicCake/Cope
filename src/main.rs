use std::{
    cmp,
    collections::HashMap,
    fs::File,
    io::{Error, Read},
    iter::Map,
    path::PathBuf,
};

use args::Args;
use clap::{builder::Str, Parser, parser};
use parse::Config;
use rand::seq::SliceRandom;

use crate::{args::PROGRAM_VERSION, tree::TreeNode};

mod args;
mod parse;
mod tree;

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
        println!(
            "Warning: Config version: {} does not match program version: {}",
            config.version, PROGRAM_VERSION
        );
    }

    let parent_messages = args.messages.unwrap_or(vec![config.default_list]);

    // The nodes
    let mut message_list: Vec<&TreeNode<Option<Vec<String>>>> = Vec::new();

    // Find and get nodes
    for messages in &parent_messages {
        let Some(found_messages) = config.message_list.get(messages) else {
            println!("Cannot find message: {}", messages);
            return Ok(());
        };
        

        message_list.push(found_messages);
    }

    // Flatten
    let mut flattened = Vec::new();
    for messages in &message_list {
        let Ok(mut res) = messages.flatten() else {
            println!("Cyclic reference detected");
            return Ok(());
        };
        flattened.append(&mut res);        
    }

    // Collect
    let mut options: Vec<String> = Vec::new();
    for node in &flattened {
        let Some(mut message) = node.data.clone() else {
            continue;
        };

        options.append(&mut message);
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
