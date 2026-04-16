use std::path::Path;
use std::process::Command;

pub fn set_wallpaper(path: &Path, setter: &str) -> Result<(), String> {
    let path_str = path.to_str().ok_or("Invalid path")?;

    let status = match setter {
        "awww" | "swww" => Command::new("awww").args(["img", path_str]).status(),
        "hyprpaper" => Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", &format!(",{}", path_str)])
            .status(),
        "feh" => Command::new("feh").args(["--bg-scale", path_str]).status(),
        "swaybg" => Command::new("swaybg").args(["-i", path_str]).status(),
        "nitrogen" => Command::new("nitrogen")
            .args(["--set-scaled", path_str])
            .status(),
        _ => return Err(format!("Unknown wallpaper setter: {}", setter)),
    };

    status
        .map_err(|e| format!("Failed to run {}: {}", setter, e))
        .and_then(|s| {
            if s.success() {
                Ok(())
            } else {
                Err(format!("{} exited with error", setter))
            }
        })
}

pub fn run_matugen(path: &Path) -> Result<(), String> {
    let path_str = path.to_str().ok_or("Invalid path")?;

    Command::new("matugen")
        .args(["image", path_str, "--source-color-index", "0"])
        .status()
        .map_err(|e| format!("Failed to run matugen: {}", e))
        .and_then(|s| {
            if s.success() {
                Ok(())
            } else {
                Err("matugen exited with error".to_string())
            }
        })
}
