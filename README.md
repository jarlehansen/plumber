# Plumber 🔧

A command-line utility for automating common system maintenance tasks, written in Rust.
The project was created for me to learn Rust while creating something useful for me personally. It is not intended for
others to use, as the features included here are very specific to my setup at home.

## Usage

```bash
# Update OS and Pi-hole (uses defaults: jarle@pi.hole:22)
plumber pihole

# Specify custom user and address
plumber pihole -u admin -a 192.168.1.100:22

# Update and reboot after completion
plumber pihole --reboot
```
