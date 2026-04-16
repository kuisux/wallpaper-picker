use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use std::path::PathBuf;

pub struct App {
    pub wallpapers: Vec<PathBuf>,
    pub selected: usize,
    pub should_quit: bool,
    pub picker: Picker,
    pub current_image: Option<StatefulProtocol>,
}

impl App {
    pub fn new(wallpapers: Vec<PathBuf>) -> Self {
        let mut picker =
            Picker::from_query_stdio().unwrap_or_else(|_| Picker::from_fontsize((8, 12)));
        let current_image = wallpapers.first().and_then(|p| load_image(&mut picker, p));

        Self {
            wallpapers,
            selected: 0,
            should_quit: false,
            picker,
            current_image,
        }
    }

    pub fn next(&mut self) {
        if !self.wallpapers.is_empty() {
            self.selected = (self.selected + 1) % self.wallpapers.len();
            self.reload_image();
        }
    }

    pub fn previous(&mut self) {
        if !self.wallpapers.is_empty() {
            if self.selected == 0 {
                self.selected = self.wallpapers.len() - 1;
            } else {
                self.selected -= 1;
            }
            self.reload_image();
        }
    }

    pub fn current(&self) -> Option<&PathBuf> {
        self.wallpapers.get(self.selected)
    }

    fn reload_image(&mut self) {
        self.current_image = self
            .current()
            .cloned()
            .and_then(|p| load_image(&mut self.picker, &p));
    }
}

fn load_image(picker: &mut Picker, path: &PathBuf) -> Option<StatefulProtocol> {
    let img = image::open(path).ok()?;
    let thumbnail = img.thumbnail(1200, 720);
    Some(picker.new_resize_protocol(thumbnail))
}
