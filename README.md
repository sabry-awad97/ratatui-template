# Ratatui Template

This is a template project for building terminal user interfaces using the Ratatui library in Rust.

## Features

- Event handling with crossterm
- Asynchronous runtime with Tokio
- Error handling with thiserror
- Logging support

## Getting Started

### Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/sabry_awad1997/ratatui-template.git
   cd ratatui-template
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the application:

   ```bash
   cargo run
   ```

## Usage

The application provides a simple terminal user interface. Here are some basic controls:

- Press 'q' or 'Esc' to quit the application
- Press 'Ctrl+C' to force quit

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/app.rs`: Main application logic and state
- `src/event.rs`: Event handling
- `src/error.rs`: Custom error types

## Dependencies

- ratatui: Terminal user interface library
- crossterm: Terminal manipulation library
- tokio: Asynchronous runtime
- thiserror: Error handling
- log: Logging facade

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Ratatui](https://github.com/ratatui-org/ratatui) for the amazing TUI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal support
