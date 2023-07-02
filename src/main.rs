use std::{
    fs::File,
    io::{Read},
    path::PathBuf, cmp, env::{current_exe, current_dir},
};

use args::Args;
use clap::Parser;
use parse::Config;
use rand::seq::SliceRandom;

use crate::args::PROGRAM_VERSION;

mod args;
mod parse;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let current_exe = current_dir().expect("This project is a binary");

    let path = args.config.unwrap_or(current_exe.join("config.json"));

    std::env::args().next().unwrap();
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

    let combo_name = args.combo.unwrap_or(config.default_combo);
    let Some(message_list_list) = config.combos.get(&combo_name) else {
        println!("Cannot find combo list {}\nFound lists: {:?}", combo_name, config.combos.keys());
        return Ok(())
    };

    // Forgive me for my naming
    let mut options: Vec<String> = Vec::new();
    for message_list_name in message_list_list {
        let Some(messages) = config.message_list.get_mut(message_list_name) else {
            println!("Could not find message list with name {}", message_list_name);
            return Ok(())
        };
        options.append(messages);
    }

    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng);

    let start_name = args.start.unwrap_or(config.default_start);
    let mut out: Vec<String> = match config.start_list.get(&start_name) {
        Some(it) => it.clone(), // idc lol
        None => {
            println!("Error: cannot get start, available starts: {:?}", config.start_list.keys());
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
