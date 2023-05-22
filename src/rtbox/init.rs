use log::{debug, info};
use std::thread::sleep;
use std::time;

use libc::{
    pid_t,
    size_t,
    sigset_t,
    c_int,
    SIG_BLOCK,
    sigprocmask,
    sigfillset,
    fork,
    wait,
    setsid,
    setpgid
};

use crate::rtbox::error::RtBoxError;

pub struct RtBoxInitState<'a> {
    pub uid: i32,
    pub gid: i32,
    pub username: &'a str,
    pub home: &'a str,
    pub shell: &'a str
}



pub trait RtBoxInitSystem {
    fn run(&self, init_state: &RtBoxInitState);
}

pub struct RtBoxInit;
impl RtBoxInit {
    pub fn new() -> Self {
        RtBoxInit {}
    }
}

impl RtBoxInitSystem for RtBoxInit {
    fn run(&self, init_state: &RtBoxInitState) {
        debug!("starting up container with init_state: {:?}", init_state.username);

        loop {

            unsafe {
                info!("we are entering unsafe code now, let there be dragons");

                let mut set: sigset_t = std::mem::zeroed();
                let mut oldset: sigset_t = std::mem::zeroed();
                let mut waitstatus: c_int = std::mem::zeroed();

                sigfillset(&mut set);
                sigprocmask(SIG_BLOCK, &mut set, &mut oldset);

                if fork() == 0 {
                    loop {
                        wait(&mut waitstatus);
                        info!("waiting for children");
                        sleep(time::Duration::from_millis(1000));
                    }
                }

                setsid();
                setpgid(0, 0);

            }

            debug!("we should execute our bootstrap script here");
            loop {
                debug!("here we should execute our host monitoring");
                sleep(time::Duration::from_millis(5000));
            }
/*

			if (fork()) for (;;) wait(&status);

			sigprocmask(SIG_UNBLOCK, &set, 0);

			setsid();
			setpgid(0, 0);
			return execve("/etc/rc", (char *[]){ "rc", 0 }, (char *[]){ 0 });
*/
        }
    }
}
