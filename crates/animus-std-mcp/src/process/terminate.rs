use std::process::{Child, Command};
use std::thread;
use std::time::{Duration, Instant};

use super::{ProcessError, ProcessScope};

#[cfg(unix)]
pub(crate) fn configure_scope(command: &mut Command) -> ProcessScope {
    use std::os::unix::process::CommandExt;

    command.process_group(0);
    ProcessScope::ProcessGroup
}

#[cfg(not(unix))]
pub(crate) fn configure_scope(_command: &mut Command) -> ProcessScope {
    ProcessScope::DirectChild
}

pub(crate) fn wait_until(
    child: &mut Child,
    timeout: Duration,
    poll_interval: Duration,
) -> Result<bool, ProcessError> {
    let started = Instant::now();
    loop {
        if child.try_wait().map_err(|_| ProcessError::Reap)?.is_some() {
            return Ok(true);
        }
        if started.elapsed() >= timeout {
            return Ok(false);
        }
        thread::sleep(poll_interval);
    }
}

pub(crate) fn terminate_owned(child: &mut Child, scope: ProcessScope) -> Result<(), ProcessError> {
    match scope {
        ProcessScope::ProcessGroup => terminate_process_group(child.id()),
        ProcessScope::DirectChild => terminate_direct_child(child),
    }
}

#[cfg(unix)]
pub(crate) fn terminate_process_group(child_id: u32) -> Result<(), ProcessError> {
    use nix::errno::Errno;
    use nix::sys::signal::{Signal, killpg};
    use nix::unistd::Pid;

    let process_group = i32::try_from(child_id).map_err(|_| ProcessError::Kill)?;
    match killpg(Pid::from_raw(process_group), Signal::SIGKILL) {
        Ok(()) | Err(Errno::ESRCH) => Ok(()),
        Err(_) => Err(ProcessError::Kill),
    }
}

#[cfg(not(unix))]
pub(crate) fn terminate_process_group(_child_id: u32) -> Result<(), ProcessError> {
    Ok(())
}

fn terminate_direct_child(child: &mut Child) -> Result<(), ProcessError> {
    match child.kill() {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::InvalidInput => Ok(()),
        Err(_) => Err(ProcessError::Kill),
    }
}
