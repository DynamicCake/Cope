use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String,

    pub default_combo: String,
    pub default_start: String,

    pub separator: String,

    pub message_list: HashMap<String, Vec<String>>,
    pub start_list: HashMap<String, Vec<String>>,
    pub combos: HashMap<String, Vec<String>>,
}

