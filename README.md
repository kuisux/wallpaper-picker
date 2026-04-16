# wallpaper-picker

A TUI wallpaper selector with matugen theming support.
<img src="https://github.com/kuisux/aperitivo/blob/main/wallpapers/showcase.png?raw=true" alt="Showcase">

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
run "wallpaper-picker" in your terminal to run :D

## Config
Create ~/.config/wallpaper-picker/config.toml:
```
wallpaper_dir = "~/Pictures/wallpapers"
wallpaper_setter = "awww"
matugen_enabled = true
```
