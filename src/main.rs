mod app;
mod config;
mod ui;
mod wallpaper;

use app::App;
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

fn scan_wallpapers(dir: &std::path::Path) -> Vec<PathBuf> {
    let extensions = ["jpg", "jpeg", "png", "webp", "gif"];

    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| extensions.contains(&e.to_lowercase().as_str()))
                        .unwrap_or(false)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn main() -> io::Result<()> {
    let config = Config::load();
    let wallpaper_dir = config.resolved_wallpaper_dir();
    let wallpapers = scan_wallpapers(&wallpaper_dir);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(wallpapers);
    let mut chosen_path: Option<std::path::PathBuf> = None;

    loop {
        terminal.draw(|f| {
            ui::draw(f, &mut app);
            ui::draw_help(f);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        if let Some(path) = app.current().cloned() {
                            if let Err(e) =
                                wallpaper::set_wallpaper(&path, &config.wallpaper_setter)
                            {
                                eprintln!("Error: {}", e);
                            }
                            if config.matugen_enabled {
                                chosen_path = Some(path);
                            }
                            app.should_quit = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    if let Some(path) = chosen_path {
        if let Err(e) = wallpaper::run_matugen(&path) {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
