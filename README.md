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

**Pi-hole management:**
```bash
# Update OS and Pi-hole (uses defaults: jarle@pi.hole:22)
plumber pihole

# Specify custom user and address
plumber pihole -u admin -a 192.168.1.100:22

# Update and reboot after completion
plumber pihole --reboot
```

**Claude Code management:**
```bash
# Show current Claude Code version
plumber claude --version

# Upgrade Claude Code (fresh install)
plumber claude --upgrade
```
