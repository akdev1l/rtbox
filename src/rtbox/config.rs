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

pub fn init() -> Result<RtBoxConfig, Box<dyn Error>> {
    let file_path = Path::new("/etc/rtbox.json");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    //let config = serde_json::from_reader(reader)?;

    let default_config: RtBoxConfig = RtBoxConfig {
        default_image: "fedora:latest".to_string(),
        socket_path: "/run/user/1000/podman/podman.sock".to_string(),
        entrypoint: vec![
            "/usr/bin/bash".to_string(), "-l".to_string(),
        ],
    };

    Ok(default_config)
}
