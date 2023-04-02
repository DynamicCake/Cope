use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub version: String, 

    pub default_list: String,
    pub default_start: String,
    
    pub separator: String,
    
    pub message_list: HashMap<String, Vec<String>>,
    pub start_list: HashMap<String, Vec<String>>,
}

