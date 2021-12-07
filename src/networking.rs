use std::process::Command;


pub fn rsync(source: &str, destination: &str) -> bool {
    Command::new("rsync")
    .arg("-avz")
    .arg(source)
    .arg(destination)
    .output()
    .is_ok()
}
