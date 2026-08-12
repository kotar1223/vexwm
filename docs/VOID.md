# Void Linux setup

Install the runtime pieces:

```bash
sudo xbps-install -S \
  rust cargo wayland wayland-protocols libinput libxkbcommon mesa \
  xorg-server-xwayland greetd foot fuzzel mako dbus
```

Build VexWM with `xbps-src` using `packaging/void/template`, or build from source with Cargo. Copy the session helper:

```bash
sudo install -Dm755 session/vexwm-session /usr/local/bin/vexwm-session
sudo install -Dm644 packaging/void/vexwm.desktop /usr/share/wayland-sessions/vexwm.desktop
```

Void uses runit, not systemd. Enable greetd if it is installed:

```bash
sudo ln -s /etc/sv/greetd /var/service/
```

Use this `/etc/greetd/config.toml`:

```toml
[terminal]
vt = 1

[default_session]
command = "tuigreet --time --remember --cmd vexwm-session"
user = "greeter"
```

For NVIDIA, install the matching Void NVIDIA package and add `nvidia_drm.modeset=1` to the kernel command line. Verify with `cat /sys/module/nvidia_drm/parameters/modeset`. Hardware-specific issues can still exist, especially with XWayland and explicit sync.
