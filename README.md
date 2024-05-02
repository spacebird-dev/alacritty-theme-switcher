# alacritty-theme-switcher

A simple tool to quickly switch between different themes for [alacritty](https://alacritty.org/index.html), with shell completion!

![./media/demo.mp4](./media/demo.mp4)

## Overview

From the Alacritty [website](https://alacritty.org/index.html)

> Alacritty is a modern terminal emulator that comes with sensible defaults, but allows for extensive configuration. By integrating with other applications, rather than reimplementing their functionality, it manages to provide a flexible set of features with high performance.

By default, Alacritty does not have support for applying different themes, color configuration is instead set in the config file.
The [`alacritty-theme`](https://github.com/alacritty/alacritty-theme) provides such theme configurations, but changing the theme has to be done manually in `alacritty.toml`.

This tool emulates theme support for Alacritty by managing the theme configuration import in using Alacritty's configuration:

```toml
# in ~/.config/alacritty/themes/themes/my-theme.toml
[colors.primary]
background = '#282c34'
foreground = '#abb2bf'
[colors.normal]
black = '#1e2127'
red = '#e06c75'
green = '#98c379'
#...

# in ~/.config/alacritty/alacritty.toml
import = [
    "~/.config/alacritty/themes/themes/my-theme-name.toml" # 👈 this line is added and managed by alacritty-theme-switcher
]
```

## Installation

### Releases

You can get the most recent release over on the [Releases](https://github.com/spacebird-dev/metallb-dyn6/releases/latest) section.

The downloaded archive (either `.zip or` `.tar.gz`) contains both the binary and shell completion scripts:

- Copy the binary to a directory that's in your path, such as `/usr/local/bin` or `~/.local/bin`
- Copy the completion scripts for your shell into the appropriate directory:
    - For Bash: `/usr/local/share/bash-completions/completions/`
    - For ZSH: `/usr/local/share/zsh/site-functions/`

### Cargo

If you have `cargo` installed, you can install this tool from `crates.io`:

`cargo install --locked alacritty-theme-switcher`

Note that you will have to install the shell completion scripts manually, see above.

### Arch Linux

There is an AUR package available: [`alacritty-theme-switcher`](https://aur.archlinux.org/packages/alacritty-theme-switcher)

### Manually

Clone this repository, then run `make build && sudo make install` to compile and install the utility.

## Usage

First, put some theme files into your theme directory (`$XDG_CONFIG/alacritty/themes/themes` by default).

Now you can switch themes by running `alacritty-theme-switcher <theme-name>`.

You can also list available themes with `alacritty-theme-switcher -l`.

## Author & License

Created by @maxhoesel

Licensed under the GPL 3 (see [the LICENSE file](./LICENSE))
