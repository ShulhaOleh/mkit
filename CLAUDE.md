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

`main.rs` is a thin entry point: it owns the command table, dispatcher, URL validation, and usage output — nothing else. Business logic lives in dedicated modules.

### Command dispatch

Commands are registered in a `COMMANDS` static table (`Cmd { name, usage, run }`). Adding a subcommand means adding one entry — no other changes needed. The URL catch-all (`mkit <repo-url>`) is validated via `is_repo_url()` before dispatch; unrecognised input prints usage and exits.

Current commands:
- **`mkit <url>`** — clones dotfiles repo (if absent) then runs the apply pipeline
- **`mkit sync`** — runs the apply pipeline on existing `~/dotfiles` without a URL
- **`mkit update`** — downloads latest binary from GitHub releases, replaces itself in-place
- **`mkit add <file> <module>`** — moves file into dotfiles module dir, replaces original with symlink
- **`mkit delete <file>`** — removes symlink, moves file back to original location

### Apply pipeline (`apply.rs`)

`apply::run` sequences: `install::packages` → `setup::run` → `link::configs`. Returns `Result<(), String>` — all exits happen in `main`.

### Dotfiles module convention

`modules::scan` reads `~/dotfiles/modules/*/`. Each module directory can have:
- `packages.dnf` — one package per line, installed via `dnf install -y`
- `install.sh` — runs as the real user (via `sudo -u $SUDO_USER`) before symlinking
- any other files — symlinked flat into `$HOME` by filename (no subdirectory support yet)

### Shared utilities (`utils.rs`)

- `home_path()` — resolves the real user's home under sudo via `SUDO_USER`, falls back to `$HOME`
- `is_root()` — checks effective UID; used by `install.rs` to decide whether to prefix `sudo`

### sudo handling

mkit is typically invoked with `sudo`. `SUDO_USER` resolves the real user's home; `SUDO_UID`/`SUDO_GID` are used for file ownership (e.g. in dotfiles module scripts).

### Update mechanism

`update::run` compares sha256 of the running binary against the downloaded one and skips if identical. It removes the old binary before copying to avoid "text file busy" errors.
