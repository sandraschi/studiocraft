# INSTALL - StudioCraft

Pure-Rust, zero runtime tax. No Python, no Node, no ports.

## Prereqs

- Rust stable (`cargo 1.99+`): `winget install Rustlang.Rustup`
- `just`: `cargo install just` or winget

## Build + verify

```powershell
cd D:\Dev\repos\studiocraft
just fmt-check
just check
just test
just run-cli
just run-app
```

Binaries: `target/debug/studiocraft-cli.exe`, `target/debug/studiocraft-app.exe`.
Release: `just build-release`. Installer: `installer/installer.nsi` (current-user, no admin).
