use libc::pid_t;
use nix::Result;
use std::process::Command;
use std::os::unix::process::CommandExt;
use nix::unistd::{fork, setpgid};
use nix::unistd::ForkResult::{Child, Parent};

pub fn spawn(cmd: String) -> Result<pid_t> {
    let pid = fork();

    match pid {
        Ok(Child) => {
            try!(setpgid(0, 0));
            Command::new(cmd)
                    .exec();
            Ok(-1)
        },
        Ok(Parent { child })  => Ok(child),
        Err(e) => Err(e)
    }
}
