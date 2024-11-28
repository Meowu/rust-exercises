use serde_json::json;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("cargo")
        .args(&["run", "--bin", "server"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start server process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");

        let request = json!({
            "method": "echo",
            "params": {"text": "Hello, JSON-RPC!"}
        });
        writeln!(stdin, "{}", request.to_string()).expect("Failed to write to stdin");
    }

    if let Some(stdout) = child.stdout.as_mut() {
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                // 打印服务端返回的响应
                println!("Server response: {}", line);
                if let Ok(response) = serde_json::from_str::<serde_json::Value>(&line) {
                    println!("Response: {}", response);
                }
            }
        }
    }

    let _ = child.wait().expect("Serve wasn't running");
}
