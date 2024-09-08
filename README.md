# Rust Video Player

A simple GUI application written in Rust that allows users to play random video files from subdirectories within their `~/Videos` directory.

## Functionality

- Displays a vertical list of buttons corresponding to subdirectories in the `~/Videos` directory.
- On clicking a button, a random video file from the selected directory is played using the system's default media player.

## Installation

### Prerequisites

Make sure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (including `cargo`)
- [Nix](https://nixos.org/download.html) (if using Nix flakes)

### Using Cargo


```bash
cargo install --git https://github.com/avrahamappel/vid-launcher
```

### Using Nix Flakes

```bash
cargo install --git https://github.com/avrahamappel/vid-launcher
```

## Usage

1. Ensure you have video files organized in subdirectories within your `~/Videos` directory.
2. Launch the application.
3. Click on any of the buttons corresponding to the subdirectories to play a random video file from that directory.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.
