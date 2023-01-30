use async_trait::async_trait;
use log::{info, warn};
use serde::{Serialize, Deserialize};
use podman_api::Podman;
use podman_api::api::{Container};
use podman_api::models::ListContainer;
use podman_api::opts::{ContainerCreateOpts, ContainerListOpts};

use crate::rtbox::engine::ContainerEngine;
use crate::rtbox::engine::Result;
use crate::rtbox::error::RtBoxError;

#[derive(Debug)]
pub struct PodmanEngine {
    podman: Podman,
}

impl PodmanEngine {
    pub fn new(podman_uri: &String) -> Self {
        Podman::new(podman_uri)
            .map(|podman| Self {
                podman: podman,
            }).unwrap()
    }
}

#[async_trait]
impl ContainerEngine for PodmanEngine {
    async fn create(
        self,
        name: String,
        image: String,
        entrypoint: Vec<String>,
        env: Vec<(String, String)>) -> Result<Container>
    {
        info!("podman-create: {} FROM {}", name, image);
        /*
        self.podman.containers()
            .create(
			   &ContainerCreateOpts::builder()
				.image(image)
                .command(vec![
                    "/usr/bin/sleep".to_string(), "infinity".to_string(),
                ])
                .env(env)
                .build(),
			).await
			.map(|container| self.podman.containers().get(container.id))
        */
        Err(RtBoxError {
            message: Some("not implemented".to_string()),
            command: None,
            root_cause: Some("not implemented".to_string()),
        })
    }

    async fn list(self, all: bool) -> Result<Vec<ListContainer>> {

        let podman_list = self.podman
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .all(all)
                    .build(),
            ).await;

        Ok(vec![])
    }

    async fn rm(self, name: String, force: bool) -> Result<()> {
        info!("podman-rm --force={} {}", force, name);

        Ok::<(), RtBoxError>(())
    }

    async fn start(self, name: String) -> Result<Container> {
        info!("podman-start {}", name);

        Ok::<Container, RtBoxError>(Container::new(self.podman, ""))
    }

    async fn get(self, name: String) -> Result<Container> {
        info!("getting container by name {}", name);

        Ok::<Container, RtBoxError>(Container::new(self.podman, ""))
    }

    async fn exec(self, name: String, command: Vec<String>, tty: bool, interactive: bool) {
        info!("podman-exec -it {} -- {:?}", name, command);
    }

    async fn export(
        self,
        container: String,
        binary_path: String,
        service_unit: String,
        application: String,
    ) -> Result<()> {
        Ok(())
    }
}
