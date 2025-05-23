# Docs

## Offline Rust std-lib docs
(optional, ~110 MB, ignored by Git)

```powershell
rustup component add rust-docs
Copy-Item "$(rustup doc --std --path)" docs/std -Recurse
``` 