use clap::Parser;

mod rtbox{
    pub mod core;
    pub mod error;
    pub mod cli;
    pub mod config;
}

use rtbox::core::{RtBox};
use rtbox::error::RtBoxError;
use rtbox::cli::{TboxCli, TboxCommands, Output};


#[tokio::main]
async fn main() {

    env_logger::init();
    rtbox::config::init();

    let args = TboxCli::parse();

    if let Ok(config) = rtbox::config::init() {

    } else {
        eprintln!("could not load config");
    }

    let output = match args.command {
        TboxCommands::Create { name, image } => {

            let image = image.unwrap_or("fedora:latest".to_string());

            if let Ok(tbox) = rtbox::core::create(name, image).await {
                Output::Create(tbox)
            } else {
                Output::Error(RtBoxError{ message: "error creating container".to_string() })
            }
        }
        TboxCommands::Rm { name, force, all } => {
            Output::Error(RtBoxError{ message: "not implemented".to_string() })
        }
        TboxCommands::List { all } => {

            if let Ok(tbox_list) = rtbox::core::list(all.unwrap_or(false)).await {
                Output::List(tbox_list)
            } else {
                Output::Error(RtBoxError{ message: "error listing containers".to_string() })
            }
        }
        TboxCommands::Run { container, cmd } => {
            println!("podman exec -it {:?} {:?}", container, cmd);
            Output::Error(RtBoxError{ message: "not implemented".to_string() })
        }
        TboxCommands::Enter { name } => {
            println!("podman exec -it {:?} /bin/bash -l", name);
            Output::Error(RtBoxError{ message: "not implemented".to_string() })
        }
    };

    if let Ok(formatted_output) = serde_json::to_string(&output) {
        println!("{}", formatted_output);
    } else {
        eprintln!("error");
    }
}
