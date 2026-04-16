# wallpaper-picker

A TUI wallpaper selector with matugen theming support.

## Dependencies
- [Rust / Cargo](https://rustup.rs/)
- [awww](https://github.com/danyspin97/awww) (or feh, hyprpaper, swaybg, nitrogen)
- [matugen](https://github.com/InioX/matugen)

## Install
```
git clone https://github.com/kuisux/wallpaper-picker
cd wallpaper-picker
cargo install --path .
```

## Config
Create ~/.config/wallpaper-picker/config.toml:

wallpaper_dir = "~/Pictures/wallpapers"
wallpaper_setter = "awww"
matugen_enabled = true
