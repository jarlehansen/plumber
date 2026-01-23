use crate::input::{ClaudeCodeAction, ClaudeCodeTarget};
use serde_json::Value;
use std::process::Command;
use ureq::http::header::AUTHORIZATION;

pub(crate) fn execute(args: &ClaudeCodeTarget) {
    match &args.action {
        ClaudeCodeAction::Version => show_current_version(),
        ClaudeCodeAction::Usage => show_current_usage(),
    }
}

fn show_current_usage() {
    let json_response = run_output(
        "security",
        &[
            "find-generic-password",
            "-s",
            "Claude Code-credentials",
            "-w",
        ],
    )
    .expect("Failed to retrieve claude code token");

    let token_json: Value =
        serde_json::from_str(&json_response).expect("Failed to parse token json");
    let token = token_json["claudeAiOauth"]["accessToken"]
        .as_str()
        .expect("Failed to extract access token from json structure");

    let usage_response = ureq::get("https://api.anthropic.com/api/oauth/usage")
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header("anthropic-version", "2023-06-01")
        .header("anthropic-beta", "oauth-2025-04-20")
        .call()
        .expect("Failed request towards the anthropic api")
        .body_mut()
        .read_to_string()
        .expect("Failed to request the usage from the anthropic api");

    let usage_json: Value =
        serde_json::from_str(&usage_response).expect("Failed to parse usage json");

    let usage = usage_json["five_hour"]["utilization"]
        .as_f64()
        .expect("Failed to extract utilization from json structure");

    let resets_at = &usage_json["five_hour"]["resets_at"]
        .as_str()
        .and_then(|s| s.get(11..16))
        .unwrap_or("N/A");

    println!("Usage {}%", usage);
    println!("Resets at {}", resets_at);
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
