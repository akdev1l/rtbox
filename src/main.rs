#[macro_use] extern crate log;
use clap::Parser;

mod rtbox{
    pub mod cli;
    pub mod config;
    pub mod engine;
    pub mod error;
    pub mod init;
    pub mod podman;
}

#[cfg(test)]
mod tests {
    pub mod engine;
}

use rtbox::cli::{TboxCli, TboxCliOutputFormat, TboxCommands, Output};
use rtbox::config::RtBoxConfig;
use rtbox::engine::{RtBoxEngine, RtBox};
use rtbox::error::RtBoxError;
use rtbox::podman::PodmanEngine;


#[tokio::main]
async fn main() {

    env_logger::init();

    let args = TboxCli::parse();
    let rtbox_config = RtBoxConfig::new("/etc/rtbox.json");
    let podman_engine = PodmanEngine::new(&rtbox_config.socket_path);
    let rtbox_engine = RtBoxEngine::new(
        &rtbox_config,
        &podman_engine,
    );

    let output = match args.command {
        TboxCommands::Create { name, image, home } => {
            debug!("rtbox-create - name: {:?}, image: {:?}, home: {:?}",
                name,
                image,
                home.unwrap_or("<no home>".to_string())
            );


            let image = image.unwrap_or("fedora:latest".to_string());

            if let Ok(tbox) = rtbox_engine.create(&name, &image).await {
                Output::Create(tbox)
            } else {
                Output::Error(RtBoxError{
                    command: Some("create".to_string()),
                    message: Some("error creating container".to_string()),
                    root_cause: Some("not implemented".to_string()),
                })
            }
        }
        TboxCommands::Rm { name, force, all } => {
            debug!("rtbox-rm - name: {:?}, force: {:?}, all: {:?}",
                name,
                force,
                all
            );

            if let Ok(tbox_rm_response) = rtbox_engine.rm(name[0].clone(), force, all).await {
                Output::Rm(tbox_rm_response)
            } else {
                Output::Error(RtBoxError{
                    command: Some("rm".to_string()),
                    message: Some("error creating container".to_string()),
                    root_cause: Some("not implemented".to_string()),
                })
            }
        }
        TboxCommands::List { all } => {
            debug!("rtbox-list - all: {:?}", all);

            match rtbox_engine.list(all).await {
                Ok(tbox_list) => Output::List(tbox_list),
                Err(e) => Output::Error(RtBoxError{
                    command: Some("rm".to_string()),
                    message: Some("error creating container".to_string()),
                    root_cause: e.root_cause,
                })
            }
        }
        TboxCommands::Run { container, cmd } => {
            debug!("rtbox-run - container: {:?}, cmd: {:?}", container, cmd);

            match rtbox_engine.run(container, cmd).await {
                Ok(rtbox_run_result) => Output::Run(rtbox_run_result),
                Err(e) => Output::Error(RtBoxError{
                    command: Some("run".to_string()),
                    message: Some("error running container".to_string()),
                    root_cause: e.root_cause,
                })
            }
        }
        TboxCommands::Enter { name } => {
            debug!("rtbox-enter - container: {:?}", name);

            Output::Error(RtBoxError{
                command: Some("enter".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Export { container, binary, service_unit, application } => {
            debug!(
                "rtbox-export - container: {:?}, binary: {:?}, service_unit: {:?}, application: {:?}",
                container,
                binary,
                service_unit,
                application,
            );

            Output::Error(RtBoxError{
                command: Some("export".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Rmi { all, force, image_name } => {
            debug!(
                "rtbox-rmi - all: {:?}, force: {:?}, image: {:?}",
                all,
                force,
                image_name
            );

            Output::Error(RtBoxError{
                command: Some("rmi".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Init { uid, gid, username, home, shell } => {
            debug!(
                "rtbox-init - gid: {:?}, home: {:?}, shell: {:?}",
                gid,
                home,
                shell
            );

            if std::process::id() != 1 {
                Output::Error(RtBoxError {
                    command: Some("init".to_string()),
                    message: Some("this is only supposed to be run as the init system of a container".to_string()),
                    root_cause:Some("we are not running as PID 1".to_string()),
                })
            } else {
                match rtbox_engine.init(uid, gid, &username, &home, &shell).await {
                    Some(e) => Output::Error(RtBoxError {
                        command: Some("init".to_string()),
                        message: Some("container init system crashed".to_string()),
                        root_cause: e.root_cause,
                    }),
                    None => Output::Init(()),
                }
            }
        }
    };

    /* We check the format selected and dispatch to the correct formatter */
    let output = match args.format {
        TboxCliOutputFormat::Human => {
            "HUMAN FORMAT".to_string()
        },
        TboxCliOutputFormat::Json => {
            if let Ok(formatted_output) = serde_json::to_string_pretty(&output) {
                formatted_output
            } else {
                "ERROR SERIALIZING".to_string()
            }
        },
    };

    println!("{}", output);
}
