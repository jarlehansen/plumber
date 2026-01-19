use crate::input::{PiHoleAction, PiHoleTarget};
use ssh2::{Channel, Session};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn execute(args: &PiHoleTarget) {
    let session = match login(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match &args.action {
        PiHoleAction::Upgrade { reboot } => {
            upgrade(&session);

            if *reboot {
                reboot_cmd(&session);
            }
        }
        PiHoleAction::Search { text } => {
            search(&session, text);
        }
    }
}

fn login(args: &PiHoleTarget) -> Result<Session, String> {
    println!("Connecting as: {}", args.username);

    let tcp = TcpStream::connect(&args.address)
        .map_err(|e| format!("Failed to connect to {}: {}", args.address, e))?;
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
    channel.exec(&build_upgrade_command()).unwrap();

    println!("Running commands to update os and pi-hole");
    println!("#########################################\n");

    read_response(&mut channel);

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

fn search(session: &Session, search_term: &str) {
    let mut channel = session.channel_session().unwrap();
    let command = build_search_command(search_term);
    channel.exec(&command).unwrap();

    println!("Running pi-hole search command");
    println!("#########################################\n");

    read_response(&mut channel);

    println!("\nSearch completed!");
}

fn build_search_command(search_term: &str) -> String {
    format!(
        "sudo grep -i '{}' /var/log/pihole/pihole.log /var/log/pihole/FTL.log 2>/dev/null",
        search_term.replace("'", "'\\''")
    )
}

fn read_response(channel: &mut Channel) {
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

    #[test]
    fn test_build_search_command() {
        let search_cmd = build_search_command("test");
        assert!(search_cmd.contains("grep"));
        assert!(search_cmd.contains(".log"));
        assert!(search_cmd.contains("test"));
    }

    #[test]
    fn test_build_search_command_with_quotes() {
        let search_cmd = build_search_command("te'st");
        assert!(search_cmd.contains("te'\\''st"));
    }
}
