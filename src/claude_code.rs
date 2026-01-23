use crate::input::{ClaudeCodeAction, ClaudeCodeTarget};
use std::process::Command;

pub(crate) fn execute(args: &ClaudeCodeTarget) {
    match &args.action {
        ClaudeCodeAction::Version => show_current_version(),
    }
}

fn show_current_version() {
    let output = run_output("claude", &["-v"]).expect("Failed to execute claude version command");
    println!("Current claude code version: {}\n", output);
}


fn run_output(app: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(app)
        .args(args)
        .output()
        .expect(&format!("Failed to execute command: {}", args.join(" ")));

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
