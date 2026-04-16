use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use ratatui_image::StatefulImage;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let items: Vec<ListItem> = app
        .wallpapers
        .iter()
        .map(|p| {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
            ListItem::new(name)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Wallpapers "))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, chunks[0], &mut list_state);

    let preview_block = Block::default().borders(Borders::ALL).title(" Preview ");
    let inner = preview_block.inner(chunks[1]);
    frame.render_widget(preview_block, chunks[1]);

    if let Some(img) = app.current_image.as_mut() {
        let image_widget = StatefulImage::default();
        frame.render_stateful_widget(image_widget, inner, img);
    } else {
        let fallback = Paragraph::new(Text::raw("No preview available"))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(fallback, inner);
    }
}

pub fn draw_help(frame: &mut Frame) {
    let help = Paragraph::new("  ↑/↓  j/k  Navigate     Enter  Select     q  Quit")
        .style(Style::default().fg(Color::DarkGray));
    let area = ratatui::layout::Rect {
        x: 0,
        y: frame.area().height.saturating_sub(1),
        width: frame.area().width,
        height: 1,
    };
    frame.render_widget(help, area);
}
