use crate::input::{ClaudeCodeAction, ClaudeCodeTarget};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn execute(args: &ClaudeCodeTarget) {
    match &args.action {
        ClaudeCodeAction::Upgrade => upgrade_claude(),
        ClaudeCodeAction::Version => show_current_version(),
    }
}

fn upgrade_claude() {
    show_current_version();

    println!("Removing the old claude code installation");
    let npm_prefix =
        run_output("npm", &["config", "get", "prefix"]).expect("Failed to get npm prefix");

    let claude_path = PathBuf::from(&npm_prefix).join("lib/node_modules/@anthropic-ai/claude-code");

    if let Ok(canonical_path) = claude_path.canonicalize() {
        let package_json = canonical_path.join("package.json");
        if package_json.exists() {
            fs::remove_dir_all(&canonical_path).expect("Failed to remove old claude code");
            println!("Removed: {}", canonical_path.display());
        } else {
            eprintln!("Warning: Directory exists but doesn't appear to be a valid installation");
        }
    } else if claude_path.exists() {
        eprintln!("Warning: Could not verify path safety, skipping deletion");
    } else {
        eprintln!("Claude code not found at: {}", claude_path.display());
    }

    println!("Cleaning npm cache");
    run("npm", &["cache", "clean", "--force"]);

    println!("Fresh install of claude code");
    run("npm", &["install", "-g", "@anthropic-ai/claude-code"]);

    show_current_version();
}

fn show_current_version() {
    let output = run_output("claude", &["-v"]).expect("Failed to execute claude version command");
    println!("Current claude code version: {}\n", output);
}

fn run(app: &str, args: &[&str]) -> bool {
    let output = Command::new(app)
        .args(args)
        .output()
        .expect(&format!("Failed to execute command: {}", args.join(" ")));

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    output.status.success()
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
