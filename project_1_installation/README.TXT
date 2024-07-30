# Install Rust

Use `rustup`, the official Rust installation tool, to install it. Run the following command in the terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
or use local shell 

```bash
sh download_rust.sh
```
# Configure the development environment

Set environment variables:

```bash
source $HOME/.cargo/env
```

# Verify the installation:

```bash
rustc --version

You can now try editing the first project:
```

```bash
cd start_example
cargo build
cargo run
```

```
You can now try our game skeleton code:
```

```bash
cd../simple_game_architecture
cargo build
cargo run
```
