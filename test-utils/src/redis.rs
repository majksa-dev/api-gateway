use std::process::Command;

pub fn start_redis(port: u16) -> Command {
    let mut cmd = Command::new("docker");
    cmd.arg("run")
        .arg("--rm")
        .arg("-p")
        .arg(format!("{}:6379", port))
        .arg("redis");
    cmd
}
