use async_trait::async_trait;
use std::char;

#[cfg(test)]
use mockall::automock;

use podman_api::api::{Container};
use podman_api::models::ListContainer;
use serde::{Serialize, Deserialize};

use crate::rtbox::error::RtBoxError;
use crate::rtbox::config::RtBoxConfig;


pub type Result<T> = std::result::Result<T, RtBoxError>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ContainerEngine {
    async fn create(
        &self,
        name: String,
        image: String,
        entrypoint: Vec<String>,
        env: Vec<(String, String)>
    ) -> Result<Container>;
    async fn list(&self, all: bool) -> Result<Vec<ListContainer>>;
    async fn rm(&self, name: String, force: bool) -> Result<()>;
    async fn exec(&self, name: String, command: Vec<String>, tty: bool, interactive: bool);
    async fn start(&self, name: String) -> Result<Container>;
    async fn inspect(&self, name: String) -> Result<Container>;
}

pub struct RtBoxEngine<'a, T: ContainerEngine> {
    pub config: &'a RtBoxConfig,
    pub container_engine: &'a T,
}

#[async_trait]
pub trait ToolbxEngine {
    async fn create(&self, name: &str, image: &str) -> Result<RtBox>;
    async fn rm(
        self,
        name: String,
        force: Option<bool>,
        all: Option<bool>
    ) -> Result<()>;
    async fn list(&self, all: Option<bool>) -> Result<Vec<RtBox>>;
    async fn exec(
        self,
        container: String,
        command: Vec<String>
    ) -> Result<RtBoxExecOutput>;
    async fn init(gid: i32, home: String, shell: String);
}

impl<'a, T: ContainerEngine> RtBoxEngine<'a, T> {
    pub fn new(rtbox_config: &'a RtBoxConfig, container_engine: &'a T) -> Self {
        Self {
            container_engine:  container_engine,
            config: rtbox_config,
        }
    }
    pub async fn create(&self, name: &str, image: &str) -> Result<RtBox> {
        debug!("rtbox-create - name: {:?}, image: {:?}", name, image);

        Ok(RtBox{
            name: name.to_string(),
            image: image.to_string(),
        })
    }
    pub async fn rm(&self, name: String, force: Option<bool>, all: Option<bool>) -> Result<()> {
        debug!("rtbox-rm - name: {:?}, force: {:?}, all: {:?}", name, force, all);

        Ok(())
    }
    pub async fn list(&self, all: Option<bool>) -> Result<Vec<RtBox>> {
        debug!("rtbox-list - all: {:?}", all);
        let container_list = self.container_engine.list(all.unwrap_or(false));

        match container_list.await {
            Ok(containers) => {
                let rtbox_list = containers.iter().
                    map(|c: &ListContainer| RtBox {
                        name: c.names.as_ref().unwrap()[0].clone(),
                        image: c.image.as_ref().unwrap().clone(),
                    })
                    .collect::<Vec<RtBox>>();

                Ok(rtbox_list)
            },
            Err(e) => Err(RtBoxError {
                root_cause: e.root_cause,
                message: None,
                command: None,
            })
        }
    }
    pub async fn run(&self, container: String, command: Vec<String>) -> Result<RtBoxExecOutput> {
        debug!("rtbox-run - container: {:?}, command: {:?}", container, command);

        Ok(RtBoxExecOutput {
            stderr: vec![char::default()],
            stdout: vec![char::default()],
            return_code: 0,
        })
    }
    pub async fn init(&self, gid: i32, home: String, shell: String) -> Option<RtBoxError> {
        debug!("rtbox-init - gid: {:?}, home: {:?}, shell: {:?}", gid, home, shell);

        None
    }
}

