# Clipboard Monitor

A simple, cross-platform Rust application that monitors your clipboard, stores up to 10 unique entries, and provides a GUI to view and manage them. Built with [iced](https://github.com/iced-rs/iced) for the user interface, [sled](https://github.com/spacejam/sled) for persistent storage, and [arboard](https://github.com/1Password/arboard) for clipboard functionality.

## Features
- Real-time clipboard monitoring.
- Stores up to 10 unique clipboard entries.
- Persistent storage in a local database (`clipboard_db/`).
- Copy any entry back to your clipboard with a button.
- Clear all entries with a single click.
- Dark-themed, scrollable GUI.

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (install via `rustup`).
- Git (optional, for cloning).
- Compatible OS: Windows, macOS, or Linux.

### Steps
1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/clipboard-monitor.git
   cd clipboard-monitor
