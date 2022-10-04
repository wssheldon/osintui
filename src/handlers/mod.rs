mod common_key_events;
mod empty;
mod error_screen;
mod home;
mod input;
mod search_result;
mod shodan;
mod unloaded;
mod virustotal;
mod virustotal_comments;
mod virustotal_results;
mod virustotal_summary;
mod virustotal_whois;

use super::app::{ActiveBlock, App, RouteId};
use crate::event::Key;

pub use input::handler as input_handler;

pub fn handle_app(key: Key, app: &mut App) {
    // First handle any global event and then move to block event
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ if key == app.user_config.keys.home => {
            app.push_navigation_stack(RouteId::Home, ActiveBlock::Input);
        }
        _ if key == app.user_config.keys.search => {
            app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
        }
        _ if key == app.user_config.keys.virustotal => {
            if app.client_config.keys.virustotal.is_empty() {
                app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::VirustotalUnloaded);
            } else {
                app.push_navigation_stack(
                    RouteId::VirustotalDetection,
                    ActiveBlock::VirustotalMenu,
                );
            }
        }
        _ if key == app.user_config.keys.shodan => {
            if app.client_config.keys.shodan.is_empty() {
                app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::ShodanUnloaded);
            } else {
                app.push_navigation_stack(RouteId::Shodan, ActiveBlock::ShodanMenu);
            }
        }
        _ => handle_block_events(key, app),
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Input => {
            input::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::Error => {
            error_screen::handler(key, app);
        }
        ActiveBlock::SearchResult => {
            search_result::handler(key, app);
        }
        ActiveBlock::VirustotalMenu => {
            virustotal::handler(key, app);
        }
        ActiveBlock::VirustotalSummary => {
            virustotal_summary::handler(key, app);
        }
        ActiveBlock::VirustotalResults => {
            virustotal_results::handler(key, app);
        }
        ActiveBlock::VirustotalWhois => {
            virustotal_whois::handler(key, app);
        }
        ActiveBlock::VirustotalComments => {
            virustotal_comments::handler(key, app);
        }
        ActiveBlock::VirustotalUnloaded => {
            unloaded::handler(key, app);
        }
        ActiveBlock::ShodanMenu => {
            shodan::handler(key, app);
        }
        ActiveBlock::ShodanServices => {
            shodan::handler(key, app);
        }
        ActiveBlock::ShodanUnloaded => {
            unloaded::handler(key, app);
        }
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        ActiveBlock::VirustotalUnloaded => {
            app.pop_navigation_stack();
        }
        ActiveBlock::ShodanUnloaded => {
            app.pop_navigation_stack();
        }
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
