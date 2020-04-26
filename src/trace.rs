use nix::unistd::Pid;
use nix::sys::ptrace;

use crate::errors::*;

pub (crate) fn trace_read(pid: Pid) -> Result<()> {
    ptrace::seize(pid, ptrace::Options::PTRACE_O_TRACESYSGOOD)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::{fork, ForkResult};
    use nix::unistd::execv;
    use nix::sys::signal::{kill, Signal};
    use std::ffi::CString;

    #[test]
    fn test_trace_read() {
        let path = "/tmp/.trace_test_sock";

        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                eprintln!("{:?}", trace_read(child));
                kill(child, Some(Signal::SIGKILL)).unwrap();
                let _ = std::fs::remove_file(&path);
            },
            Ok(ForkResult::Child) => {
                execv(&CString::new("/bin/nc").unwrap(), &[&CString::new("/bin/nc").unwrap(), &CString::new("-U").unwrap(), &CString::new(path).unwrap(), &CString::new("-l").unwrap()]).unwrap();
            },
            Err(_) => panic!("Fork failed"),
        }
    }
}
