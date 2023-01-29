use log::{info, warn};
use serde::{Serialize, Deserialize};
use podman_api::Podman;
use podman_api::api::{Container};
use podman_api::models::ListContainer;
use podman_api::opts::{ContainerCreateOpts, ContainerListOpts};

use crate::rtbox::error::PodmanError;

#[derive(Debug)]
pub struct PodmanEngine {
    podman: Podman,
}

type Result<T> = std::result::Result<T, PodmanError>;

impl PodmanEngine {
    pub fn new(podman_uri: &String) -> Result<Self> {
        Podman::new(podman_uri)
            .map(|podman| Self {
                podman: podman,
            })
            .map_err(|e| PodmanError {
                method: "new".to_string(),
                message: e.to_string(),
            })
    }
    pub async fn create(self, name: &str, image: &str, entrypoint: Vec<&str>, env: Vec<(&str, &str)>) -> Result<Container> {
        info!("podman-create: {} FROM {}", name, image);

        self.podman.containers()
            .create(
			   &ContainerCreateOpts::builder()
				.image(image)
                .command(entrypoint)
                .env(env)
                .build(),
			).await
			.map(|container| self.podman.containers().get(container.id))
			.map_err(|e| PodmanError {
				method: "create".to_string(),
				message: e.to_string(),
			})
    }

    pub async fn list(self, all: bool) -> Result<Vec<ListContainer>> {

        let podman_list = self.podman
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .all(all)
                    .build(),
            ).await;

        let rtbox_list = podman_list
            .map_err(|e| PodmanError {
                method: "list".to_string(),
                message: e.to_string(),
            });

        rtbox_list
    }

    pub async fn rm(self, name: String, force: bool) -> Result<()> {
        info!("podman-rm --force={} {}", force, name);

        Ok::<(), PodmanError>(())
    }

    pub async fn start(self, name: String) -> Result<Container> {
        info!("podman-start {}", name);

        Ok::<Container, PodmanError>(Container::new(self.podman, ""))
    }

    pub async fn get(self, name: String) -> Result<Container> {
        info!("getting container by name {}", name);

        Ok::<Container, PodmanError>(Container::new(self.podman, ""))
    }

    pub async fn exec(self, name: String, command: Vec<String>, tty: bool, interactive: bool) {
        info!("podman-exec -it {} -- {:?}", name, command);
    }
}
