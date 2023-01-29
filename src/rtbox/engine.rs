use std::char;

use log::{info, warn};
use serde::{Serialize, Deserialize};
use podman_api::Podman;
use podman_api::opts::{ContainerCreateOpts, ContainerListOpts};

use crate::RtBoxError;
use crate::rtbox::config::RtBoxConfig;
use crate::rtbox::podman::PodmanEngine;

#[derive(Serialize, Deserialize, Debug)]
pub struct RtBox {
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RtBoxExecOutput {
    stdout: Vec<char>,
    stderr: Vec<char>,
    return_code: u64,
}

pub struct Engine {
    pub config: RtBoxConfig,
    pub podman_engine: PodmanEngine,
}

impl Engine {
    pub fn new(rtbox_config_path: &String) -> Result<Self> {
        let rtbox_config = RtBoxConfig::new(&rtbox_config_path);

        PodmanEngine::new(&rtbox_config.socket_path)
            .map(|podman| Self {
                podman_engine: podman,
                config: rtbox_config,
            })
            .map_err(|e| RtBoxError {
                error: e.to_string(),
            })
    }
    pub async fn create(self, name: &str, image: &str) -> Result<RtBox> {
        let podman_result = self.podman_engine.create(
            name.clone(),
            image.clone(),
            vec!["/usr/bin/bash"],
            vec![("TOOLBOX_PATH", "toolbox")],
        ).await
        .map(|container| RtBox {
            name: name.to_string(),
            image: image.to_string(),
        })
        .map_err(|e| RtBoxError {
            error: e.to_string(),
        });

        podman_result
    }
    pub async fn rm(self, name: String, force: Option<bool>, all: Option<bool>) -> Result<()> {
        Ok(())
    }
    pub async fn list(self, all: Option<bool>) -> Result<Vec<RtBox>> {
        let podman_result = self.podman_engine.list(all.unwrap_or(false)).await;

        let rtbox_list = podman_result
            .map(|container_list| container_list.iter()
                .map(|c| {
                    let names = c.names.clone().unwrap_or(vec!["null".to_string()]);
                    let image = c.image.clone().unwrap_or("null".to_string());
                    RtBox{
                        name: names[0].clone(),
                        image: image,
                    }
            }).collect())
            .map_err(|e| RtBoxError {
                error: e.to_string(),
            });

        rtbox_list
    }
    pub async fn exec(self, container: String, command: Vec<String>) -> Result<RtBoxExecOutput> {
        Ok(RtBoxExecOutput {
            stderr: vec![char::default()],
            stdout: vec![char::default()],
            return_code: 0,
        })
    }
}

type Result<T> = std::result::Result<T, RtBoxError>;

pub async fn create(name: String, image: String) -> Result<RtBox> {
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    match podman
        .containers()
        .create(
			&ContainerCreateOpts::builder()
                .name(&name)
                .image(&image)
                .command(
                    ["/usr/bin/bash", "-l"]
                )
                .env([
                    ("app", "web"),
                ])
                .build(),
        ).await
    {
        Ok(created_container) => {
            info!("container created {}", created_container.id);
            Ok(RtBox {
                name: name,
                image: image,
            })
        }
        Err(error) => {
            Err(RtBoxError{ error: "error calling podman API".to_string() })
        }
	}
}

pub async fn list(all: bool) -> Result<Vec<RtBox>> {
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    if let Ok(containers) = podman
        .containers()
        .list(
            &ContainerListOpts::builder()
                .all(all)
                .build(),
        ).await {

        let tbox_list = containers.iter().map(|container| {
            let image = container.image.clone().unwrap_or_default();

            let name = container.names.clone().unwrap_or_default();
            RtBox { name: name[0].clone(), image: image.to_string() }
        }).collect();

        Ok(tbox_list)
    } else {
        eprintln!("error!");
        Err(RtBoxError{ error: "error calling podman api".to_string() })
    }
}
pub fn rm(name: String) {
    println!("tbox-rm: {:?}", name);
}

pub fn enter(name: String) {
}

pub fn run(name: String, cmd: Vec<String>) {
    println!("tbox-exec: {:?} - {:?}", name, cmd);
}
