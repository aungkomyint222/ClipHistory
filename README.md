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
   git clone https://github.com/aungkomyint222/ClipHistory.git
   cd ClipHistory
   ```

2. **Build and Run**
   ```bash
   cargo build --release
   cargo run --release
   ```

## Usage

1. Launch the application.
2. The app will automatically start monitoring your clipboard.
3. Copy any text - it will appear in the GUI automatically.
4. Click on any entry to copy it back to your clipboard.
5. Use the "Clear All" button to remove all entries.
6. The app will automatically save entries between sessions.

## Configuration

The application stores its database in:
- Windows: `%APPDATA%/clipboard_db/`
- Linux: `~/.local/share/clipboard_db/`
- macOS: `~/Library/Application Support/clipboard_db/`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Aung Ko Myint - [GitHub](https://github.com/aungkomyint222)

Project Link: [https://github.com/aungkomyint222/ClipHistory](https://github.com/aungkomyint222/ClipHistory)
