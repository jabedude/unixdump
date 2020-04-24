use std::env;

use log::*;

fn usage() -> ! {
    eprintln!("unixdump <path to unix socket>");
    std::process::exit(1);
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }

    let target_pid = args[1].parse::<i32>().expect("Error getting PID argument");
    info!("targ pid: {}", target_pid);
}
