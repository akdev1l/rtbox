use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::rtbox::cli::{TboxCli, TboxCliOutputFormat, TboxCommands, Output};

pub trait OutputFormatter {
    fn format(&self, command_output: &Output) -> Option<String>;
}

pub struct HumanFormatter {
}
impl OutputFormatter for HumanFormatter {
    fn format(&self, command_output: &Output) -> Option<String> {
        match command_output {
            Output::Create(rtbox) => {
                let formatted_output = format!("\
                    Successfully created '{}' using image {}. \
                    To enter please run:\n\
                    > rtbox enter {}", rtbox.name, rtbox.image, rtbox.name);

                Some(formatted_output)
            },
            Output::List(rtbox_list) => {
                let formatted_list = rtbox_list
                    .iter()
                    .map(|rtbox| format!("{:<12} | {:<20} | {:<20} | {}", rtbox.id.chars().take(12).collect::<String>(), rtbox.name, rtbox.status, rtbox.image))
                    .collect::<Vec<String>>()
                    .join("\n");

                let header = format!("{:<12} | {:<20} | {:<20} | {}\n", "ID", "NAME", "STATUS", "IMAGE");

                let formatted_output = header + &formatted_list;

                Some(formatted_output)
            },
            Output::Error(error) => Some(format!("{:?}", error)),
            _ => None,
        }
    }
}


pub struct JsonFormatter {
}
impl OutputFormatter for JsonFormatter {
    fn format(&self, command_output: &Output) -> Option<String> {
        Some(serde_json::to_string_pretty(command_output).unwrap())
    }
}
