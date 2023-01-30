use std::vec::Vec;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Serialize, Deserialize};

use crate::RtBox;
use crate::RtBoxError;
use crate::rtbox::engine::RtBoxExecOutput;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TboxCliOutputFormat {
    Json,
    Human,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TboxLogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
    ALL,
}

#[derive(Debug, Parser)]
#[clap(name = "tbox")]
#[clap(about = "A rust implementation of toolbx", long_about = None)]
pub struct TboxCli {
    #[clap(short, long)]
    verbose: Option<i32>,
    /// Automatically answer yes to all questions
    #[clap(short, long)]
    #[arg(short = 'y')]
    #[arg(default_value_t = false)]
    assume_yes: bool,
    /// Set the logging level
    #[clap(long)]
    #[arg(default_value_t = TboxLogLevel::INFO)]
    #[arg(value_enum)]
    log_level: TboxLogLevel,
    /// Set the output format
    #[clap(short, long)]
    #[arg(value_enum)]
    #[arg(default_value_t = TboxCliOutputFormat::Human)]
    format: TboxCliOutputFormat,
    /// Do not actually execute API calls
    #[clap(long)]
    #[arg(default_value_t = false)]
    dry_run: bool,
    /// Subcommand to run
    #[command(subcommand)]
    pub command: TboxCommands,
}

#[derive(Debug, Subcommand)]
pub enum TboxCommands {
    /// Create a rtbox container
    #[command(arg_required_else_help = true)]
    Create {
        /// Container name, will also be used as part of the hostname
        name: String,
        /// Image to use as base for the container
        #[clap(short, long)]
        image: Option<String>,
        /// Set a custom HOME directory for the container
        #[clap(short, long)]
        #[arg(short = 'H')]
        home: Option<String>,
    },
    /// Remove a rtbox container
    #[command(arg_required_else_help = true)]
    Rm {
        /// Container to remove
        name: Vec<String>,
        /// Remove container even if it is currently running
        #[clap(short, long)]
        force: Option<bool>,
        /// Remove all rtbox containers
        #[clap(short, long)]
        all: Option<bool>,
    },
    /// Remove a rtbox container image
    #[command(arg_required_else_help = true)]
    Rmi {
        /// Remove all rtbox container images
        #[clap(short, long)]
        #[arg(default_value_t = false)]
        all: bool,
        /// Remove rtbox container images even if running containers are using it
        #[clap(short, long)]
        #[arg(default_value_t = false)]
        force: bool,
        /// Name of image to remove
        image_name: Option<String>,
    },
    /// Execute a command inside a rtbox container
    #[command(arg_required_else_help = true)]
    Run {
        /// Container name
        #[clap(short, long)]
        container: String,
        /// Command to execute
        cmd: Vec<String>,
    },
    /// Enter into a new shell session inside a rtbox container
    #[command(arg_required_else_help = true)]
    Enter {
        /// Container to enter into
        name: String,
    },
    /// List all rtbox containers
    List {
        /// Show all rtbox containers even if they not actively running
        #[clap(short, long)]
        all: Option<bool>,
    },
    /// Export an application, service or binary from a rtbox container to the host
    #[command(arg_required_else_help = true)]
    Export {
        /// Container from where to export the application
        #[clap(short, long)]
        container: String,
        /// Path to an executable that will be exported (must exist inside the container)
        #[clap(short, long)]
        binary: Option<String>,
        /// Service unit name that will be exported (must exist inside the container)
        #[clap(short, long)]
        service_unit: Option<String>,
        /// Desktop application name that will be exported (must exist inside the container)
        #[clap(short, long)]
        application: Option<String>,
    },
    /// Used to initialize rtbox containers
    #[command(arg_required_else_help = true, hide = true)]
    Init {
        #[clap(long)]
        gid: i32,
        #[clap(long)]
        home: String,
        #[clap(long)]
        shell: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Output {
    Create(RtBox),
    List(Vec<RtBox>),
    Run(RtBoxExecOutput),
    Rm(()),
    Error(RtBoxError),
}
