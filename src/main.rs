use std::env::args;

use p_kill::killer;

fn main() {
    let args: Vec<String> = args().collect();

    let killer = killer {
        port: String::from(&args[1]),
    };

    killer.kill_process();
}
