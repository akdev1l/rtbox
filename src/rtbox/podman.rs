use async_trait::async_trait;
use log::{debug};
use podman_api::Podman;
use podman_api::ApiVersion;
use podman_api::api::{Container};
use podman_api::models::ListContainer;
use podman_api::opts::{ContainerCreateOpts, ContainerListOpts, ContainerListFilter};

use crate::rtbox::engine::ContainerEngine;
use crate::rtbox::engine::Result;
use crate::rtbox::error::RtBoxError;

#[derive(Debug)]
pub struct PodmanEngine {
    podman: Podman,
}

impl PodmanEngine {
    pub fn new(podman_uri: &String) -> Self {
        Podman::new_versioned(podman_uri, ApiVersion::new(3, 0, 0))
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

        const TOOLBX_LABEL: &str = "com.github.containers.toolbox";

        let podman_list_response = self.podman
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .all(all)
                    .filter(
                        vec![ContainerListFilter::LabelKey(TOOLBX_LABEL.to_string())],
                    )
                    .build(),
            ).await;

        podman_list_response.map_err(|e| RtBoxError {
            root_cause: Some(e.to_string()),
            command: Some("list".to_string()),
            message: None,
        })
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
