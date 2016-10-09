use nix;
use libc::pid_t;
use nix::errno::Errno;
use std::process::Command;
use nix::unistd::{fork, setpgid};
use std::os::unix::process::CommandExt;
use nix::unistd::ForkResult::{Child, Parent};
use nix::sys::wait::{self, waitpid, WaitStatus};
use parser::EnvVar;

pub fn parse_command(cmd: &str, args: &Vec<String>, envs: &Vec<EnvVar>) -> Command {
    let mut command = Command::new(cmd);
    for argv in args {
        command.arg(argv);
    }

    for env in envs {
        command.env(&env.0, &env.1);
    }

    command
}

pub fn spawn_command(cmd: &mut Command) -> nix::Result<pid_t> {
    let pid = fork();
    match pid {
        Ok(Child) => {
            try!(setpgid(0, 0));
            cmd.exec();
            Ok(-1)
        }
        Ok(Parent { child }) => Ok(child),
        Err(e) => Err(e),
    }
}

pub enum ReapState {
    Next,
    Exit(i32),
}

pub fn reap_zombies(child_pid: pid_t) -> nix::Result<ReapState> {
    loop {
        match waitpid(-1, Some(wait::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, code)) => {
                trace!("exited pid: {:?} code: {:?}", pid, code);
                if child_pid == pid {
                    return Ok(ReapState::Exit(code as i32));
                }
            }
            Ok(WaitStatus::Signaled(pid, sig, exit_with_error)) => {
                trace!("signaled pid: {:?} sig: {:?}", pid, sig);
                if child_pid == pid {
                    return Ok(ReapState::Exit(if exit_with_error { 128 } else { 0 }));
                }
            }
            Ok(WaitStatus::Stopped(_, _)) => {
                return Ok(ReapState::Next);
            }
            Ok(WaitStatus::Continued(_)) => {
                return Ok(ReapState::Next);
            }
            Ok(WaitStatus::StillAlive) => {
                return Ok(ReapState::Next);
            }
            Err(nix::Error::Sys(Errno::ECHILD)) => {
                trace!("No more child");
                return Ok(ReapState::Exit(-1));
            }
            Err(e) => return panic!("Failed to reap child process due to {:?}", e),
        }
    }
}
