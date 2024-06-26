use std::{
    cmp, collections::HashMap, env::current_dir, fmt::Debug, fs::File, io::Read, path::PathBuf, process::{ExitCode, Termination}
};

use args::Args;
use clap::Parser;
use parse::Config;
use rand::seq::SliceRandom;

use crate::args::PROGRAM_VERSION;

mod args;
mod parse;

fn main() -> anyhow::Result<(), ProgramError> {
    let args = Args::parse();

    let current_exe = current_dir().expect("This project is a binary");

    let path = args.config.unwrap_or(current_exe.join("config.json"));
    let Ok(mut config_file) = File::open(&path) else {
        return Err(ProgramError::CannotOpenConfig(path));
    };

    let mut config_content = String::new();
    if let Err(_) = config_file.read_to_string(&mut config_content) {
        return Err(ProgramError::CannotReadConfig(path));
    }
    let mut config: Config = serde_json::from_str(&config_content)
        .map_err(|err| ProgramError::CannotParseConfig(err))?;

    if config.version != PROGRAM_VERSION {
        return Err(ProgramError::VersionMismatch(config.version));
    }

    let combo_name = args.combo.unwrap_or(config.default_combo);
    // message lists means it is a list of
    let Some(message_list_names) = config.combos.get(&combo_name) else {
        return Err(ProgramError::CannotFindComboList(config.combos, combo_name));
    };

    // Forgive me for my naming
    let mut options: Vec<String> = Vec::new();
    for message_list_name in message_list_names {
        let Some(messages) = config.message_list.get_mut(message_list_name) else {
            return Err(ProgramError::CannotFindMessageList(
                message_list_name.clone(),
            ));
        };
        options.append(messages);
    }

    let mut rng = rand::thread_rng();
    options.shuffle(&mut rng);

    let start_name = args.start.unwrap_or(config.default_start);
    let mut out: Vec<String> = match config.start_list.get(&start_name) {
        Some(it) => it.clone(), // idc lol
        None => {
            return Err(ProgramError::CannotFindStart(
                config.start_list,
                start_name.clone(),
            ));
        }
    };
    out.append(&mut options);
    let new_len = cmp::min(args.length, out.len());
    out.resize_with(new_len, || {
        // This should never be reached
        panic!("There was a problem resizing the array size");
    });

    let out = out.join(&config.separator);

    println!("{}", out);

    Ok(())
}

#[derive(thiserror::Error)]
enum ProgramError {
    #[error("Cannot open config file {0}")]
    CannotOpenConfig(PathBuf),
    #[error("Cannot read config file {0}")]
    CannotReadConfig(PathBuf),
    #[error("Cannot parse config file {0}")]
    CannotParseConfig(serde_json::error::Error),
    #[error("Config version: {0} does not match program version: {PROGRAM_VERSION}")]
    VersionMismatch(String),
    #[error("Cannot find combo list {1} Lists: {:#?}", .0.keys())]
    CannotFindComboList(HashMap<String, Vec<String>>, String),
    #[error("Cannot find message list with name {0}")]
    CannotFindMessageList(String),
    #[error("Cannot find start {1} Starts: {:#?}", .0.keys())]
    CannotFindStart(HashMap<String, Vec<String>>, String),
}

// This is kind of cursed implementation, but debug will never realistically be used
impl Debug for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Termination for ProgramError {
    fn report(self) -> std::process::ExitCode {
        ExitCode::FAILURE
    }
}
