use crate::input::{PiHoleAction, PiHoleTarget};
use ssh2::Session;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn execute(args: &PiHoleTarget) {
    match &args.action {
        PiHoleAction::Upgrade { reboot } => {
            let session = login(args).expect("Authentication failed.");
            upgrade(&session);

            if *reboot {
                reboot_cmd(&session);
            }
        }
    }
}

fn login(args: &PiHoleTarget) -> Result<Session, String> {
    println!("Connecting as: {}", args.username);

    let tcp = TcpStream::connect(&args.address).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    println!("Password:");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();
    session
        .userauth_password(&args.username, &password)
        .unwrap();

    if session.authenticated() {
        Ok(session)
    } else {
        Err("Authentication failed".to_string())
    }
}

fn upgrade(session: &Session) {
    let mut channel = session.channel_session().unwrap();
    channel
        .exec(&build_upgrade_command())
        .unwrap();

    println!("Running commands to update os and pi-hole");
    println!("#########################################\n");

    let mut buffer = [0; 1024];
    loop {
        match channel.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                println!("{}", String::from_utf8_lossy(&buffer[..n]));
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("\nConnection error: {}", e);
                break;
            }
        }
    }

    println!("\nUpgrade completed!");
}

fn build_upgrade_command() -> String {
    "sudo apt update && sudo apt full-upgrade -y && sudo pihole -up".to_string()
}

fn reboot_cmd(session: &Session) {
    println!("Rebooting...");
    let mut channel = session.channel_session().unwrap();
    channel.exec("sudo reboot").unwrap();
    println!("Pi-hole is now rebooting");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_upgrade_command() {
        let upgrade_cmd = build_upgrade_command();
        assert!(upgrade_cmd.contains("update"));
        assert!(upgrade_cmd.contains("full-upgrade"));
        assert!(upgrade_cmd.contains("pihole"));
    }
}