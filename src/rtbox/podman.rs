use async_trait::async_trait;
use log::{debug};
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
        &self,
        name: String,
        image: String,
        entrypoint: Vec<String>,
        env: Vec<(String, String)>) -> Result<Container>
    {
        debug!("podman-create - name: {:?}", name);
        debug!("FROM {:?}", image);
        debug!("ENTRYPOINT {:?}", entrypoint);
        debug!("ENV: {:?}", env);

        let _podman_create_opts = ContainerCreateOpts::builder()
            .image(image)
            .command(entrypoint)
            .env(env)
            .build();
            
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

    async fn list(&self, all: bool) -> Result<Vec<ListContainer>> {

        let _podman_list = self.podman
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .all(all)
                    .build(),
            ).await;

        Ok(vec![])
    }

    async fn rm(&self, name: String, force: bool) -> Result<()> {
        debug!("podman-rm - name: {:?}, force: {:?}", name, force);

        Ok::<(), RtBoxError>(())
    }

    async fn start(&self, name: String) -> Result<Container> {
        debug!("podman-start - name: {:?}", name);

        Ok::<Container, RtBoxError>(Container::new(self.podman.clone(), ""))
    }

    async fn inspect(&self, name: String) -> Result<Container> {
        debug!("podman-inspect - name: {:?}", name);

        Ok::<Container, RtBoxError>(Container::new(self.podman.clone(), ""))
    }

    async fn exec(&self, name: String, command: Vec<String>, tty: bool, interactive: bool) {
        debug!("podman-exec - name: {:?}, tty: {:?}, interactive: {:?}", name, tty, interactive);
        debug!("command: {:?}", command);
    }
}
