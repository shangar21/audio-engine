use std::process::{Command, Stdio, Child};
use std::io;
use std::io::{BufRead, BufReader};
use tokio::task;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub type SessionMap = Arc<Mutex<HashMap<String, Child>>>;

pub fn run_command(label: &str, command: &str) {
    let label = label.to_string();
    let command = command.to_string();

    task::spawn_blocking(move || {
        println!("[{}] Starting: {}", label, command);

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to spawn command");

        if let Some(stdout) = child.stdout.take() {
            let label_clone = label.clone();
            std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    println!("[{} stdout] {}", label_clone, line);
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let label_clone = label.clone();
            std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().flatten() {
                    eprintln!("[{} stderr] {}", label_clone, line);
                }
            });
        }

        let status = child.wait().expect("Failed to wait on child");
        println!("[{}] Process exited with: {}", label, status);
    });
}

pub fn launch_command(label: &str, command: &str) -> io::Result<Child> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
}

