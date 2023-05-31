use std::process::Command;

pub fn get_pid_by_port(port: &str) -> Option<u32> {
    println!("port: {}", port);
    let lsof_output = Command::new("lsof")
        .arg("-t")
        .arg("-i")
        .arg(format!(":{}", port))
        .output()
        .expect("No pid found");

    println!("{:#?}", lsof_output);

    if lsof_output.stdout.is_empty() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&lsof_output.stdout);
    let lines: Vec<&str> = output_str.split('\n').collect();

    Some(lines[0].parse::<u32>().unwrap())
}

pub fn kill_process(pid: u32) {
    Command::new("kill")
        .arg("-9")
        .arg(format!("{}", pid))
        .output()
        .expect("Cannot kill process");
}
