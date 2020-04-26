use std::env;
use std::fs;
use std::path::PathBuf;

use log::*;

mod errors;
mod trace;

fn usage() -> ! {
    eprintln!("unixdump <path to unix socket>");
    std::process::exit(1);
}

fn main() {
    env_logger::init();
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }

    let target_socket: PathBuf = fs::canonicalize(args.remove(1))
                                    .expect("Error canonicalizing path");
    info!("targ socket: {:?}", target_socket);
}
