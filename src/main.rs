use std::env::args;

use p_kill::Killer;

fn main() {
    let args: Vec<String> = args().collect();

    let killer = Killer {
        port: args[1].parse::<u32>().unwrap(),
    };

    killer.kill_process_by_port();
}
