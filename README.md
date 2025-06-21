# Pattern Forge Sandbox

A Bevy 0.16 playground for experiments.

## Getting Started

Run the game in debug mode:

```bash
cargo run
```

The **World Inspector** is compiled only in debug builds. It relies on
`EguiPlugin` being added before `WorldInspectorPlugin` in `main.rs`. When
running in debug, open the inspector by pressing **F1**.

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
