use async_trait::async_trait;
use std::char;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

use podman_api::api::{Container};
use podman_api::models::ListContainer;
use serde::{Serialize, Deserialize};

use crate::rtbox::init::{RtBoxInit, RtBoxInitState, RtBoxInitSystem};
use crate::rtbox::error::RtBoxError;
use crate::rtbox::config::RtBoxConfig;

extern "C" {
    fn geteuid() -> u32;
    fn getegid() -> u32;
}

pub type Result<T> = std::result::Result<T, RtBoxError>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RtBox {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
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
        name: &str,
        image: &str,
        entrypoint: Vec<String>,
        env: Vec<(&str, String)>,
        mounts: Vec<&(&str, &str, &str)>,
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
        info!("creating toolbox {} based on {}", name, image);

        let home_dir = std::env::var("HOME").unwrap();
        let host_mounts = [
            /* (source, destination, options) */
            ("/", "/run/host", "rbind"),
            ("/etc/machine-id", "/etc/machine-id", "rbind:ro"),
            ("/run/libvirt", "/run/libvirt", "rbind"),
            ("/run/systemd/journal", "/run/systemd/journal", "rbind"),
            ("/run/systemd/resolve", "/run/systemd/resolve", "rbind"),
            ("/run/systemd/sessions", "/run/systemd/sessions", "rbind"),
            ("/run/systemd/system", "/run/systemd/system", "rbind"),
            ("/run/systemd/users", "/run/systemd/users", "rbind"),
            ("/run/udev/data", "/run/udev/data", "rbind"),
            ("/run/udev/tags", "/run/udev/tags", "rbind"),
            ("/tmp", "/tmp", "rbind"),
            ("/var/lib/flatpak", "/var/lib/flatpak", "rbind:ro"),
            ("/var/lib/libvirt", "/var/lib/libvirt", "rbind"),
            ("/var/lib/systemd/coredump", "/var/lib/systemd/coredump", "rbind:ro"),
            ("/var/log/journal", "/var/log/journal", "rbind:ro"),
            ("/var/mnt", "/var/mnt", "rbind:rslave"),
            (&home_dir, &home_dir, "rbind")
        ];
        let available_host_mounts = host_mounts.len();
        debug!("host_mounts: {:?}", host_mounts);

        let host_mounts: Vec<_> = host_mounts.iter()
            .filter(|mount| Path::new(mount.0).exists())
            .collect();
        debug!("active mounts: {:?}", host_mounts);
        info!("detected {}/{} active host mounts", host_mounts.len(), available_host_mounts);

        let euid = unsafe {
            geteuid()
        };
        let egid = unsafe {
            getegid()
        };
        let entrypoint = vec![
            format!("/run/host{}", std::env::current_exe().unwrap().display()),
            "init".to_string(),
            "--uid".to_string(), euid.to_string(),
            "--gid".to_string(), egid.to_string(),
            "--home".to_string(), home_dir.to_string(),
            "--username".to_string(), "akdev".to_string(),
            "--shell".to_string(), "/bin/bash".to_string(),
        ];
        info!("setting entry point to {}", entrypoint[0]);

        let container_env = vec![
            "DBUS_SESSION_BUS_ADDRESS",
            "DESKTOP_SESSION",
            "DISPLAY",
            "GDMSESSION",
            "GDM_LANG",
            "HOME",
            "HOSTNAME",
            "LANG",
            "LOGNAME",
            "LSCOLORS",
            "LS_COLORS",
            "QT_IM_MODULE",
            "QT_WAYLAND_DECORATION",
            "SESSION_MANAGER",
            "TERM",
            "USER",
            "USERNAME",
            "WAYLAND_DISPLAY",
            "XCURSOR_SIZE",
            "XCURSOR_THEME",
            "XDG_CURRENT_DESKTOP",
            "XDG_DATA_DIRS",
            "XDG_MENU_PREFIX",
            "XDG_RUNTIME_DIR",
            "XDG_SESSION_CLASS",
            "XDG_SESSION_DESKTOP",
            "XDG_SESSION_TYPE",
        ];
        let total_vars = container_env.len();

        let container_env: Vec<_> = container_env
            .into_iter()
            .map(|env_var| (env_var, std::env::var(env_var)))
            .filter(|env_var| env_var.1.is_ok())
            .map(|env_var| (env_var.0, env_var.1.unwrap()))
            .collect();
        info!("detected {}/{} environment variables to preserve", container_env.len(), total_vars);

        let container_create = self.container_engine.create(
            name,
            image,
            entrypoint,
            container_env,
            host_mounts,
        );

        match container_create.await {
            Ok(container) => Ok(RtBox{
                name: name.to_string(),
                image: image.to_string(),
                id: container.id().to_string(),
                status: false.to_string(),
            }),
            Err(err) => Err(err),
        }
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
                        id: c.id.as_ref().unwrap().clone(),
                        name: c.names.as_ref().unwrap()[0].clone(),
                        image: c.image.as_ref().unwrap().clone(),
                        status: c.exited.unwrap().to_string(),
                    })
                    .collect::<Vec<RtBox>>();

                debug!("rtbox list: {:?}", rtbox_list);
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
    pub async fn init<'b>(
        &self,
        uid: i32,
        gid: i32,
        username: &'b str,
        home: &'b str,
        shell: &'b str
    ) -> Option<RtBoxError> {
        debug!("rtbox-init - gid: {:?}, home: {:?}, shell: {:?}", gid, home, shell);

        let rtbox_init_state = RtBoxInitState {
            uid: uid,
            gid: gid,
            home: home,
            username: username,
            shell: shell,
        };

        let rtbox_init: RtBoxInit = RtBoxInit::new();

        rtbox_init.run(&rtbox_init_state);
        None
    }
}

