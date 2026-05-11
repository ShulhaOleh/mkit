# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build                  # build debug binary
cargo build --release        # build release binary
cargo clippy -- -D warnings  # lint (CI enforces zero warnings)
```

No tests exist yet. CI runs `cargo build` + `cargo clippy` on every push to main.

Release is a musl-linked static binary built automatically on push to main via GitHub Actions and published to the `latest` GitHub release tag.

## Architecture

`main.rs` is the entry point and orchestrator. Subcommands map directly to modules:

- **`mkit <url>`** — calls `apply()`: clones dotfiles repo → `install::packages` → `setup::run` → `link::configs`
- **`mkit update`** — `update::run`: downloads latest binary from GitHub releases, replaces itself in-place
- **`mkit add <file> <module>`** — `add::run`: moves file into dotfiles module dir, replaces original with symlink
- **`mkit delete <file>`** — `delete::run`: removes symlink, moves file back to original location

### Dotfiles module convention

`modules::scan` reads `~/dotfiles/modules/*/` directories. Each module directory can have:
- `packages.dnf` — one package name per line, installed via `dnf install -y`
- `install.sh` — runs as the real user (via `sudo -u $SUDO_USER`) before symlinking
- any other files — symlinked into `$HOME` by filename (no subdirectory structure)

### sudo handling

mkit is typically invoked with `sudo`. Two patterns appear throughout the codebase:
- **Real user home**: `SUDO_USER` env var is used to resolve `~/dotfiles` and symlink targets, avoiding `/root`
- **Real user identity**: `setup::run` runs `install.sh` as `$SUDO_USER` via `sudo -u`; uid/gid for file ownership uses `SUDO_UID`/`SUDO_GID` env vars

### Update mechanism

`update::run` compares sha256 of the running binary against the downloaded one and skips if identical. It deletes the old binary before copying to avoid "text file busy" errors when replacing an in-use executable.
