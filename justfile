set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

# StudioCraft automation (Pure-Rust ArtCraft pattern)

default:
    @just --list

# Strict ArtCraft lint gate across all workspace members
check:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo clippy --workspace --all-targets --all-features -- -D warnings

# Unit + integration tests
test:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo test --workspace

# Formatting gate
fmt-check:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo fmt --all --check

# All gates in one command
ci: fmt-check check test

# Start the MCP server over stdio (a client normally does this; useful for a manual probe)
run-mcp:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo run -q -p studiocraft-mcp -- mcp

# CLI smoke: status, caps, then init+open a temp project
run-cli:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo run -q -p studiocraft-mcp -- status; cargo run -q -p studiocraft-mcp -- caps

# Headless L3 parity checks (no window)
run-app:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo run -q -p studiocraft-app -- --smoke

# Production build
build-release:
    $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"; cargo build --release --workspace
