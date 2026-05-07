# mkit

Bootstrap your Linux environment from a dotfiles repo.

```bash
mkit https://github.com/you/dotfiles
```

Clones the repo, installs packages, symlinks configs.

## Dotfiles structure

```
modules/
  zsh/
    .zshrc
    packages.dnf   # one package per line
    install.sh     # optional, runs before symlinking
```