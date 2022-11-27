use log::{info, warn};
use serde::{Serialize, Deserialize};
use podman_api::Podman;
use podman_api::opts::{ContainerCreateOpts, ContainerListOpts};

use crate::RtBoxError;

#[derive(Serialize, Deserialize, Debug)]
pub struct RtBox {
    pub name: String,
    pub image: String,
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
            Err(RtBoxError{ message: "error calling podman API".to_string() })
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
        Err(RtBoxError{ message: "error calling podman api".to_string() })
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
