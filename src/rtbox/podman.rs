use std::vec::Vec;
use std::process::Command;

use podman_api::Podman;

pub fn exec(container: String, args: Vec<String>) {

}

pub fn start(container: String) {

}

pub fn stop(container: String, force: bool) {
}

pub fn kill(container: String, force: bool, all: bool) {
}

pub fn rm(container: String, force: bool, all: bool) {

}

pub fn ps(all: bool) {
    let podman = Podman::unix("/run/user/1000/podman/podman.sock");

    if let Err(e) = podman
        .containers()
        .list()
    {
        eprintln!("{}", e);
    }
}
