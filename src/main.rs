use std::env::args;

use p_kill::{get_pid_by_port, kill_process};

fn main() {
    let args: Vec<String> = args().collect();

    println!("args: {:?}", args[1]);

    if let Some(pid) = get_pid_by_port(&args[1]) {
        println!("PID of the process listening on port {}: {}", args[1], pid);
        kill_process(pid);
    } else {
        println!("No process found listening on port {}", args[1]);
    }
}
