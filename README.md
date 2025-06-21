# Pattern Forge Sandbox

A Bevy 0.16 playground for experiments.

## Getting Started

Run the game in debug mode:

```bash
cargo run
```

When running in debug, open the **World Inspector** by pressing **F1**.

For a production-ready build:

```bash
cargo run --release
```

## Offline Rust std-lib docs

(optional, ~110 MB, ignored by Git)

```powershell
rustup component add rust-docs
Copy-Item "$(rustup doc --std --path)" docs/std -Recurse
```
