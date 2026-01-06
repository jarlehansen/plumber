mod claude_code;
mod input;
mod pi_hole;

use crate::input::Targets;

fn main() {
    let args = input::parse_args();

    println!("\n╔═══════════════════════════╗");

    match args.command {
        Targets::Pihole(args) => {
            println!("║   π-hole  Plumber  🔧      ║");
            println!("╚═══════════════════════════╝\n");
            pi_hole::execute(&args);
        }
        Targets::ClaudeCode(args) => {
            println!("║   Claude Code  🤖          ║");
            println!("╚═══════════════════════════╝\n");
            claude_code::execute(&args);
        }
    }
}
