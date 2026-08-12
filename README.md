# VexWM

A lightweight, keyboard-driven Wayland compositor inspired by dwm, vxwm and niri.

VexWM is designed around one simple idea: the compositor should stay small, predictable and configurable. Desktop shells, panels, launchers and widgets are optional external components. QuickShell works as a separate shell, not as a hard dependency.

> **Status:** early development scaffold. The repository layout, configuration format and project direction are ready, but compositor functionality must be implemented incrementally before this is production-ready.

## Goals

- Native Wayland compositor written in Rust.
- Simple KDL configuration inspired by niri.
- Live configuration reload.
- Tiling layout with floating and fullscreen support.
- XWayland support for legacy applications.
- External shell support: QuickShell, Waybar, Eww or no shell.
- greetd-compatible sessions.
- Mesa and NVIDIA support through wlroots/Smithay protocols and DRM modesetting.
- Low idle memory target: approximately 100-250 MB for the compositor and core services. A 1 GB total desktop guarantee is impossible because browsers, Qt shells and portals are separate processes.

## Repository layout

```text
vexwm/
├── src/                  # compositor source
├── config/default.kdl    # documented default configuration
├── docs/                 # design and compatibility notes
├── Cargo.toml
├── LICENSE
└── README.md

vexwm-dots/
├── config/config.kdl     # opinionated desktop configuration
├── shell/shell.qml       # optional QuickShell shell
├── scripts/              # session helpers
└── README.md
```

## Configuration

VexWM searches for configuration in this order:

1. `VEXWM_CONFIG`
2. `$XDG_CONFIG_HOME/vexwm/config.kdl`
3. `~/.config/vexwm/config.kdl`
4. `/etc/vexwm/config.kdl`

The file is intended to be readable without learning a programming language:

```kdl
input {
    keyboard {
        layout "us,ru"
        options "grp:win_space_toggle"
    }
}

layout {
    gaps 8
    outer-gaps 12
    main-ratio 0.55
    smart-gaps true
}

binds {
    Mod+Return spawn "foot"
    Mod+D spawn "fuzzel"
    Mod+Q close
    Mod+Shift+E exit
    Mod+R reload-config
}

autostart {
    command "mako"
    command "quickshell -c vexwm"
}
```

## Build from source

Install Rust using [rustup](https://rustup.rs/), then install the platform dependencies for Smithay and Wayland from your distribution. After that:

```bash
git clone https://github.com/YOUR_USERNAME/vexwm.git
cd vexwm
cargo build --release
install -Dm755 target/release/vexwm ~/.local/bin/vexwm
install -Dm644 config/default.kdl ~/.config/vexwm/config.kdl
```

The current scaffold validates the configuration and provides the project foundation. Renderer, input, output management, protocol globals and window management should be implemented in the milestones described in `docs/ROADMAP.md`.

## NVIDIA notes

VexWM cannot remove driver bugs. For NVIDIA, use a current proprietary driver and enable DRM kernel modesetting:

```text
nvidia_drm.modeset=1
```

Verify it with:

```bash
cat /sys/module/nvidia_drm/parameters/modeset
```

The compositor should prefer explicit sync when the driver and backend expose it. Keep `WLR_NO_HARDWARE_CURSORS=1` as a troubleshooting fallback, not a default requirement. Some applications may still have XWayland, Qt or explicit-sync issues on particular driver versions.

## greetd

Example `/etc/greetd/config.toml`:

```toml
[terminal]
vt = 1

[default_session]
command = "tuigreet --time --remember --cmd vexwm-session"
user = "greeter"
```

Install `vexwm-session` to `~/.local/bin` or `/usr/local/bin` and make it executable. Do not run the compositor as root.

## Install on common distributions

Package names vary, but the usual dependencies are:

### Arch Linux

```bash
sudo pacman -S --needed base-devel rust wayland wayland-protocols \
  libinput mesa xorg-xwayland greetd tuigreet foot fuzzel mako
```

### Fedora

```bash
sudo dnf install rust cargo wayland-devel wayland-protocols-devel \
  libinput-devel mesa-libEGL-devel libxkbcommon-devel xorg-x11-server-Xwayland \
  greetd foot fuzzel mako
```

### Debian/Ubuntu

```bash
sudo apt install cargo rustc libwayland-dev wayland-protocols libinput-dev \
  libxkbcommon-dev mesa-common-dev xwayland greetd tuigreet foot fuzzel mako
```

## Uploading to GitHub

Create two empty repositories on GitHub named `vexwm` and `vexwm-dots`, then run:

```bash
cd vexwm
git init
git branch -M main
git add .
git commit -m "Initial VexWM scaffold"
git remote add origin git@github.com:YOUR_USERNAME/vexwm.git
git push -u origin main

cd ../vexwm-dots
git init
git branch -M main
git add .
git commit -m "Initial VexWM desktop configuration"
git remote add origin git@github.com:YOUR_USERNAME/vexwm-dots.git
git push -u origin main
```

For HTTPS remotes, replace the remote URLs with `https://github.com/YOUR_USERNAME/...`.

## License

MIT. See `LICENSE`.

## Void Linux

Void is supported as a first-class target. See `docs/VOID.md` and `packaging/void/`. Void uses runit rather than systemd, so the session helper deliberately avoids systemd-only commands.

## Current implementation status

This release is a **development scaffold**, not a finished daily-driver compositor. It loads and validates KDL configuration, includes the session and packaging structure, and documents the implementation roadmap. The Smithay backend, rendering, input, output management, window lifecycle and protocol support still need to be completed before calling it production-ready. Do not replace a working compositor or display manager with it yet.
