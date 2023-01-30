use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RtBoxConfig {
    pub default_image: String,
    pub socket_path: String,
}

impl Default for RtBoxConfig {
    fn default() -> Self {
        let rtbox_podman_socket_path = env::var("RTBOX_PODMAN_SOCKET")
            .unwrap_or("unix:///var/run/docker.sock".to_string());
        Self {
            default_image: "fedora:latest".to_string(),
            socket_path: rtbox_podman_socket_path,
        }
    }
}

impl RtBoxConfig {
    pub fn new(config_path: &str) -> Self {
        let file_path = Path::new(&config_path);
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);

            let file_config = serde_json::from_reader(reader);


            file_config.unwrap_or(Self::default())
        } else {
            Self::default()
        }
    }
}
