# Void Linux package

Copy `template` to `void-packages/srcpkgs/vexwm/template`, replace the maintainer and release URL, then build:

```bash
git clone https://github.com/void-linux/void-packages
cd void-packages
cp /path/to/vexwm/packaging/void/template srcpkgs/vexwm/template
./xbps-src pkg vexwm
sudo xi vapoursynth # replace with the generated package path shown by xbps-src
sudo xbps-install --repository=hostdir/binpkgs vexwm
```

The exact dependency names can differ between Void repositories. Check them with `xbps-query -Rs 'wayland|libinput|xkbcommon'` before building.
