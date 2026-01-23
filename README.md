# Plumber 🔧

A command-line utility for automating common system maintenance tasks, written in Rust.
The project was created for me to learn Rust while creating something useful for me personally. It is not intended for
others to use, as the features included here are very specific to my setup at home.

## Installation & Development

**Install globally:**
```bash
cargo install --path .  # installs to ~/.cargo/bin/plumber
```

**Development workflow:**
```bash
cargo run -- <args>           # build and run (no install needed)
cargo install --path .        # update installed binary after changes
```

## Usage

**Pi-hole 🕳️:**
```bash
# Upgrade OS and Pi-hole (uses defaults: jarle@pi.hole:22)
plumber pihole upgrade

# Specify custom user and address
plumber pihole --username admin --address 192.168.1.100:22 upgrade

# Upgrade and reboot after completion
plumber pihole upgrade --reboot

# Combine flags (flags can go before or after the action)
plumber pihole -u admin -a 192.168.1.100:22 upgrade --reboot

# Search Pi-hole logs (searches pihole.log and FTL.log)
plumber pihole search "blocked"
plumber pihole search "error"
plumber pihole search "192.168.1.50"

# Search is case-insensitive by default
plumber pihole search "ERROR"  # finds "error", "Error", "ERROR", etc.
```

**Claude Code 🤖:**
```bash
# Show current Claude Code version
plumber claude version

# Show current usage statistics
plumber claude usage
```
