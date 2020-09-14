use std::process::Command;
use std::thread::spawn;

fn main() {
  Command::new("yarn")
    .arg("build")
    .current_dir("../app-web")
    .spawn()
    .unwrap();
}
