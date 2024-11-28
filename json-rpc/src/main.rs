use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

fn handle_request(request: &Value) -> Value {
    let method = request["method"].as_str();
    let params = &request["params"];
    println!("Params: {}", params);
    match method {
        Some("echo") => params.clone(),
        _ => json!({"error": "Method not found"}),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(request) = serde_json::from_str::<Value>(&line) {
                let response = handle_request(&request);
                if let Ok(response_str) = serde_json::to_string(&response) {
                    writeln!(stdout, "{}", response_str).unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
    }
}
