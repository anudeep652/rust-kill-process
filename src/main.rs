use std::env::args;

use p_kill::{get_pid_by_port, kill_process};

pub struct killer {
    port: String,
}

impl killer {
    pub fn kill_process(&self) {
        if let Some(pid) = get_pid_by_port(&self.port) {
            println!(
                "PID of the process listening on port {}: {}",
                self.port, pid
            );
            kill_process(pid);
        } else {
            println!("No process found listening on port {}", self.port);
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    let killer = killer {
        port: String::from(&args[1]),
    };

    killer.kill_process();
}
