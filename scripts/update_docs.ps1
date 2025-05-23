param(
    [string]$Tag = "v0.16.0"
)

$ErrorActionPreference = "Stop"

Push-Location "docs/bevy-src"
git fetch --tags
git checkout $Tag
Pop-Location

cargo clean -p bevy
cargo doc --package bevy --no-deps --release   # matches fixed Bash script

Remove-Item -Recurse -Force "docs/bevy-html"
Copy-Item "target/doc" "docs/bevy-html" -Recurse

"$Tag | $(rustc --version --short) | $(Get-Date -Format yyyy-MM-dd)" |
    Out-File -Encoding utf8 "docs/VERSION.txt"

Write-Host "Docs updated to Bevy $Tag" 