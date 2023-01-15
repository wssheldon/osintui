extern crate unicode_width;
use super::super::app::{ActiveBlock, App, RouteId};
use super::super::network::IoEvent;
use crate::event::Key;
use std::{convert::TryInto, net::IpAddr, str::FromStr};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

// Handle event when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Enter => {
            let input_str: String = app.input.iter().collect();
            process_input(app, input_str);
        }
        Key::Ctrl('k') => {
            app.input.drain(app.input_idx..app.input.len());
        }
        Key::Ctrl('u') => {
            app.input.drain(..app.input_idx);
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('l') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('w') => {
            if app.input_cursor_position == 0 {
                return;
            }
            let word_end = match app.input[..app.input_idx].iter().rposition(|&x| x != ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let word_start = match app.input[..word_end].iter().rposition(|&x| x == ' ') {
                Some(index) => index + 1,
                None => 0,
            };
            let deleted: String = app.input[word_start..app.input_idx].iter().collect();
            let deleted_len: u16 = UnicodeWidthStr::width(deleted.as_str()).try_into().unwrap();
            app.input.drain(word_start..app.input_idx);
            app.input_idx = word_start;
            app.input_cursor_position -= deleted_len;
        }
        Key::End | Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }
        Key::Home | Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left | Key::Ctrl('b') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Right | Key::Ctrl('f') => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                app.input_cursor_position += compute_character_width(next_c);
            }
        }
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            app.input_cursor_position += compute_character_width(c);
        }
        Key::Backspace | Key::Ctrl('h') => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                app.input_cursor_position -= compute_character_width(last_c);
            }
        }
        Key::Delete | Key::Ctrl('d') => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Home));
        }
        _ => {}
    }
}

fn process_input(app: &mut App, input: String) {
    // Don't do anything if there is no input
    if input.is_empty() {
        return;
    }

    let ip = refang(input);

    if !is_ip_addr(ip.to_owned()) {
        app.is_input_error = true
    }

    if !app.client_config.keys.censys_secret.is_empty() {
        app.dispatch(IoEvent::Censys(ip.to_owned()));
    }

    if !app.client_config.keys.shodan.is_empty() {
        app.dispatch(IoEvent::Shodan(ip.to_owned()));
    }

    if !app.client_config.keys.virustotal.is_empty() {
        app.dispatch(IoEvent::VirusTotal(ip.clone()));
        app.dispatch(IoEvent::VirustotalComments(ip));
    }

    app.push_navigation_stack(RouteId::SearchResult, ActiveBlock::SearchResult);
}

fn is_ip_addr(input: String) -> bool {
    IpAddr::from_str(&input).is_ok()
}

fn refang(input: String) -> String {
    input.replace(&['[', ']'], "")
}

fn compute_character_width(character: char) -> u16 {
    UnicodeWidthChar::width(character)
        .unwrap()
        .try_into()
        .unwrap()
}
