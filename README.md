# wallpaper-picker

A TUI wallpaper selector with matugen theming support.
<img src="https://github.com/kuisux/aperitivo/blob/main/wallpapers/showcase.png?raw=true" alt="Showcase">

## Dependencies
- [Rust / Cargo](https://rustup.rs/)
- [awww](https://github.com/danyspin97/awww) (or feh, hyprpaper, swaybg, nitrogen)
- [matugen](https://github.com/InioX/matugen)

## Install
On arch
```
yay -S wallpaper-picker-rs
```
or
```
paru -S wallpaper-picker-rs
```
other distros or through cargo
```
git clone https://github.com/kuisux/wallpaper-picker
cd wallpaper-picker
cargo install --path .
```
if installed through aur its "wallpaper-picker-rs" to run else its just "wallpaper-picker", make sure ~/.cargo/bin is in path if you installed through cargo

## Config
Create ~/.config/wallpaper-picker/config.toml:
```
wallpaper_dir = "~/Pictures/wallpapers"
wallpaper_setter = "awww"
matugen_enabled = true
```
