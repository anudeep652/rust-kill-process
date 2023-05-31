use rust_kill_process::get_pid_by_port;

fn main() {
    let port = 8000;

    if let Some(pid) = get_pid_by_port(port) {
        println!("PID of the process listening on port {}: {}", port, pid);
    } else {
        println!("No process found listening on port {}", port);
    }
}
