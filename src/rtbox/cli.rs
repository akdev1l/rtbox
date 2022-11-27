use std::vec::Vec;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

use crate::RtBox;
use crate::RtBoxError;

#[derive(Debug, Parser)]
#[clap(name = "tbox")]
#[clap(about = "A rust implementation of toolbx", long_about = None)]
pub struct TboxCli {
    #[clap(short, long)]
    verbose: Option<i32>,
    #[clap(short, long)]
    json: Option<bool>,
    #[command(subcommand)]
    pub command: TboxCommands,
}

#[derive(Debug, Subcommand)]
pub enum TboxCommands {
    #[command(arg_required_else_help = true)]
    Create {
        name: String,
        #[clap(short, long)]
        image: Option<String>,
    },
    #[command(arg_required_else_help = true)]
    Rm {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        force: Option<bool>,
        #[clap(short, long)]
        all: Option<bool>,
    },
    #[command(arg_required_else_help = true)]
    Run {
        #[clap(short, long)]
        container: String,
        cmd: Vec<String>,
    },
    #[command(arg_required_else_help = true)]
    Enter {
        name: String,
    },
    List {
        #[clap(short, long)]
        all: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Output {
    Create(RtBox),
    List(Vec<RtBox>),
    Error(RtBoxError),
}
