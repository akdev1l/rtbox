use std::boxed::Box;
use std::vec::Vec;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RtBoxConfig {
    pub default_image: String,
    pub socket_path: String,
    pub entrypoint: Vec<String>,
}

impl Default for RtBoxConfig {
    fn default() -> Self {
        Self {
            default_image: "fedora:latest".to_string(),
            socket_path: "unix:///run/user/1000/podman/podman.sock".to_string(),
            entrypoint: vec![
                "/usr/bin/bash".to_string(), "-l".to_string(),
            ],
        }
    }
}

impl RtBoxConfig {
    pub fn new(config_path: &String) -> Self {
        let file_path = Path::new(config_path);
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);

            let file_config = serde_json::from_reader(reader);


            file_config.unwrap_or(Self::default())
        } else {
            Self::default()
        }
    }
}
