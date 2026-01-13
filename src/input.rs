use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Targets,
}

#[derive(Parser, Debug)]
pub enum Targets {
    #[command(name = "pihole", about = "Pi-hole management")]
    Pihole(PiHoleTarget),

    #[command(name = "claude", about = "Claude code management")]
    ClaudeCode(ClaudeCodeTarget),
}

#[derive(Parser, Debug)]
pub struct PiHoleTarget {
    #[command(subcommand)]
    pub action: PiHoleAction,

    #[arg(
        short,
        long,
        default_value = "jarle",
        help = "Username for SSH authentication to pi.hole server"
    )]
    pub username: String,

    #[arg(
        short,
        long,
        default_value = "pi.hole:22",
        help = "Address of the remote host running the pi-hole installation"
    )]
    pub address: String,
}

#[derive(Parser, Debug)]
pub enum PiHoleAction {
    #[command(name = "upgrade", about = "Upgrade the OS and pi-hole packages")]
    Upgrade {
        #[arg(short, long, help = "Reboot the system after upgrade")]
        reboot: bool,
    },

    #[command(name = "search", about = "Search pi-hole logs for a specific term")]
    Search {
        #[arg(help = "The search term to look for in the logs")]
        text: String,
    }
}

#[derive(Parser, Debug)]
pub struct ClaudeCodeTarget {
    #[command(subcommand)]
    pub action: ClaudeCodeAction,
}

#[derive(Parser, Debug)]
pub enum ClaudeCodeAction {
    #[command(name = "upgrade", about = "Upgrade the local claude code instance")]
    Upgrade,

    #[command(name = "version", about = "The current claude code version")]
    Version,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
