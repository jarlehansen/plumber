mod input;
mod pi_hole;

use crate::input::Commands;

fn main() {
    let args = input::parse_args();

    match args.command {
        Commands::Pihole(args) => {
            println!("\n╔═══════════════════════════╗");
            println!("║   π-hole  Plumber  🔧      ║");
            println!("╚═══════════════════════════╝\n");
            pi_hole::execute(&args);
        }
    }
}
