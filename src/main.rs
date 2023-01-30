use clap::Parser;

mod rtbox{
    pub mod engine;
    pub mod error;
    pub mod cli;
    pub mod config;
    pub mod podman;
}

#[cfg(test)]
mod tests {
    pub mod engine;
}

use rtbox::cli::{TboxCli, TboxCommands, Output};
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

            match rtbox_engine.list(all).await {
                Ok(tbox_list) => Output::List(tbox_list),
                Err(e) => Output::Error(RtBoxError{
                    command: Some("rm".to_string()),
                    message: Some("error creating container".to_string()),
                    root_cause: Some("not implemented".to_string()),
                })
            }
        }
        TboxCommands::Run { container, cmd } => {
            println!("podman exec -it {:?} {:?}", container, cmd);
            Output::Error(RtBoxError{
                command: Some("run".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Enter { name } => {
            println!("podman exec -it {:?} /bin/bash -l", name);
            Output::Error(RtBoxError{
                command: Some("enter".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Export { container, binary, service_unit, application } => {
            println!("export {}:{}", container, binary.unwrap_or("null".to_string()));
            Output::Error(RtBoxError{
                command: Some("export".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Rmi { all, force, image_name } => {
            Output::Error(RtBoxError{
                command: Some("rmi".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
        TboxCommands::Init { gid, home, shell } => {
            Output::Error(RtBoxError{
                command: Some("init".to_string()),
                message: Some("error creating container".to_string()),
                root_cause: Some("not implemented".to_string()),
            })
        }
    };

    if let Ok(formatted_output) = serde_json::to_string(&output) {
        println!("{}", formatted_output);
    } else {
        eprintln!("error");
    }
}
