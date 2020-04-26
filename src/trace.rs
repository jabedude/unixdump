use nix::unistd::Pid;
use nix::sys::ptrace;

use crate::errors::*;

pub (crate) fn trace_read(pid: Pid) -> Result<()> {
    eprintln!("tracing : {:?}", pid);
    ptrace::seize(pid, ptrace::Options::PTRACE_O_TRACESYSGOOD | ptrace::Options::PTRACE_O_TRACEEXEC | ptrace::Options::PTRACE_O_TRACEEXIT)?;
    eprintln!("seized : {:?}", pid);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::{fork, ForkResult};
    use nix::unistd::execv;
    use nix::sys::wait::waitpid;
    use nix::sys::ptrace;
    use nix::sys::signal::{kill, Signal};
    use std::thread;
    use std::time::Duration;
    use std::ffi::CString;

    use rusty_fork::rusty_fork_id;
    use rusty_fork::rusty_fork_test;
    use rusty_fork::rusty_fork_test_name;

    //rusty_fork_test! {
    #[test]
    fn test_trace_read() {
        let path = "/tmp/.trace_test_sock";

        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                //thread::sleep(Duration::from_millis(9000));
                //let res = waitpid(child, None).unwrap();
                //eprintln!("res {:?}", res);

                eprintln!("{:?}", trace_read(child));
                kill(child, Some(Signal::SIGKILL)).unwrap();
                std::fs::remove_file(&path);
            },
            Ok(ForkResult::Child) => {
                //ptrace::traceme().unwrap();

                execv(&CString::new("/bin/nc").unwrap(), &[&CString::new("/bin/nc").unwrap(), &CString::new("-U").unwrap(), &CString::new("/tmp/.trace_test_sock").unwrap(), &CString::new("-l").unwrap()]).unwrap();
                eprintln!("ret");
            },
            Err(_) => panic!("Fork failed"),
        }
    }
    //}
}
