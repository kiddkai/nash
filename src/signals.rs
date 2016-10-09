use libc::pid_t;
use nix::{self, Result};
use nix::sys::signal::{self, SigSet};

pub fn init() -> nix::Result<signal::SigAction> {
    extern "C" fn handler(_: i32) {}
    let sig_action = signal::SigAction::new(signal::SigHandler::Handler(handler),
                                            signal::SaFlags::empty(),
                                            signal::SigSet::empty());

    unsafe { signal::sigaction(signal::Signal::SIGCHLD, &sig_action) }
}

pub enum ForwardState {
    ChildDead,
    Forwarded,
}

pub fn wait_and_forward(pid: pid_t, should_kill_group: bool) -> Result<ForwardState> {
    let mask = SigSet::all();
    let signal = mask.wait();

    match signal {
        Ok(s) => {
            match s {
                signal::Signal::SIGCHLD => Ok(ForwardState::ChildDead),
                _ => {
                    signal::kill(if should_kill_group { -pid } else { pid }, s)
                        .map(|_| ForwardState::Forwarded)
                }
            }
        }
        Err(e) => Err(e),
    }
}
