use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[command(name = "pihole")]
    Pihole(PiHoleCmd),
}

#[derive(Parser, Debug)]
pub struct PiHoleCmd {
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

    #[arg(short, long, help = "Reboot the system after upgrade")]
    pub reboot: bool,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
