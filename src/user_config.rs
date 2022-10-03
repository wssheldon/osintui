use crate::event::Key;
use std::path::PathBuf;
use tui::style::Color;

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub active: Color,
    pub banner: Color,
    pub error_border: Color,
    pub error_text: Color,
    pub hint: Color,
    pub hovered: Color,
    pub inactive: Color,
    pub selected: Color,
    pub text: Color,
    pub header: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            active: Color::Cyan,
            banner: Color::LightRed,
            error_border: Color::Red,
            error_text: Color::LightRed,
            hint: Color::Yellow,
            hovered: Color::Magenta,
            inactive: Color::Gray,
            selected: Color::LightCyan,
            text: Color::Reset,
            header: Color::Reset,
        }
    }
}

#[derive(Clone)]
pub struct KeyBindings {
    pub back: Key,
    pub home: Key,
    pub search: Key,
    pub submit: Key,
    pub virustotal: Key,
    pub shodan: Key,
}

#[derive(Clone)]
pub struct UserConfig {
    pub keys: KeyBindings,
    pub theme: Theme,
    pub path_to_config: Option<UserConfigPaths>,
}

#[derive(Clone)]
pub struct UserConfigPaths {
    pub config_file_path: PathBuf,
}

impl UserConfig {
    pub fn new() -> UserConfig {
        UserConfig {
            theme: Default::default(),
            keys: KeyBindings {
                home: Key::Char('h'),
                back: Key::Char('q'),
                search: Key::Char('/'),
                submit: Key::Enter,
                virustotal: Key::Char('v'),
                shodan: Key::Char('s'),
            },
            path_to_config: None,
        }
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self::new()
    }
}
