# mkit

Bootstrap your Linux environment from a dotfiles repo.

```bash
curl -fsSL https://raw.githubusercontent.com/ShulhaOleh/mkit/main/bootstrap.sh | sudo bash
mkit https://github.com/you/dotfiles
```

## Commands

```bash
mkit <repo-url>          # clone repo, install packages, symlink configs
mkit update              # update mkit to the latest version
mkit add <file> <module> # start tracking a config file
mkit delete <file>       # stop tracking a config file
```

## Dotfiles structure

```
modules/
  zsh/
    .zshrc
    packages.dnf   # one package per line
    install.sh     # optional, runs before symlinking
```
