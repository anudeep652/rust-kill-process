use std::env::args;

use p_kill::killer;

fn main() {
    let args: Vec<String> = args().collect();

    let killer = killer {
        port: args[1].parse::<u32>().unwrap(),
    };

    killer.kill_process_by_port();
}
