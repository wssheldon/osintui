use tui::style::{Color, Style};

pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::LightYellow),
        (false, true) => Style::default().fg(Color::LightYellow),
        _ => Style::default().fg(Color::Gray),
    }
}

// `percentage` param needs to be between 0 and 1
pub fn get_percentage_width(width: u16, percentage: f32) -> u16 {
    let padding = 3;
    let width = width - padding;
    (f32::from(width) * percentage) as u16
}
