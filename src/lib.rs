use std::io::{self, Write};
use std::process::Command;

#[doc(hidden)]
pub fn kill(raw_pid: u32) {
    let output = Command::new("kill")
        .args(["-s", "TERM", &raw_pid.to_string()])
        .output()
        .expect("kill -s TERM command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
