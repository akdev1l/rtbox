use async_trait::async_trait;
use log::{debug};
use podman_api::Podman;
use podman_api::ApiVersion;
use podman_api::api::{Container};
use podman_api::models::{ListContainer, Namespace, ContainerMount};
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
        Podman::new_versioned(podman_uri, ApiVersion::new(3, None, None))
            .map(|podman| Self {
                podman: podman,
            }).unwrap()
    }
}

#[async_trait]
impl ContainerEngine for PodmanEngine {
    async fn create(
        &self,
        name: &str,
        image: &str,
        entrypoint: Vec<String>,
        env: Vec<(&str, String)>,
        mounts: Vec<&(&str, &str, &str)>,
    ) -> Result<Container> {
        debug!("podman-create - name: {:?}", name);
        debug!("FROM {:?}", image);
        debug!("ENTRYPOINT {:?}", entrypoint);
        debug!("ENV: {:?}", env);

        let labels = vec![
            ("com.github.containers.toolbox", "true")
        ];

        let mounts = mounts
            .iter()
            .map(|mount| ContainerMount{
                source: Some(mount.0.to_string()),
                destination: Some(mount.1.to_string()),
                options: Some(mount.2.split(":").map(|it| it.to_string()).collect()),
                _type: None,
                gid_mappings: None,
                uid_mappings: None,
            })
            .collect::<Vec<ContainerMount>>();

        let podman_create_opts = ContainerCreateOpts::builder()
            .image(image.to_string())
            .command(entrypoint)
            .env(env)
            .mounts(mounts)
            .hostname(format!("{}.host", name))
            .name(name.to_string())
            .selinux_opts(vec!["disable"])
            .work_dir("/var/home/akdev")
            .labels(labels)
            .user_namespace(Namespace{
                nsmode: Some("keep-id".to_string()),
                value: None,
            })
            .build();
        debug!("podman_create_opts: {:?}", podman_create_opts);
            
        self.podman.containers()
            .create(&podman_create_opts)
            .await
			.map(|container| self.podman.containers().get(container.id))
            .map_err(|err| RtBoxError {
                command: Some("create".to_string()),
                message: Some(err.to_string()),
                root_cause: Some("podman".to_string())
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
