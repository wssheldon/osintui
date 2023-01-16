mod censys;
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

use super::app::{ActiveBlock, App, ResultStatus, RouteId};
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
        _ if key == app.user_config.keys.censys => {
            if app.client_config.keys.censys_secret.is_empty()
                && app.client_config.keys.censys_id.is_empty()
            {
                app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::CensysUnloaded);
            } else if matches!(app.censys.status, ResultStatus::NotFound) {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::CensysNotFound)
            } else if matches!(app.censys.status, ResultStatus::NotQueried) {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::CensysNotQueried)
            } else {
                // Reset the menu index with switching to the view
                app.censys.menu_index = 0;

                // Switch to the main Censys view
                app.push_navigation_stack(RouteId::Censys, ActiveBlock::CensysMenu);
            }
        }
        _ if key == app.user_config.keys.shodan => {
            if app.client_config.keys.shodan.is_empty() {
                app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::ShodanUnloaded);
            } else if matches!(app.shodan.status, ResultStatus::NotFound) {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::ShodanNotFound)
            } else if matches!(app.shodan.status, ResultStatus::NotQueried) {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::ShodanNotQueried)
            } else {
                // Reset the menu index with switching to the view
                app.shodan.menu_index = 0;

                // Switch to the main Shodan view
                app.push_navigation_stack(RouteId::Shodan, ActiveBlock::ShodanMenu);
            }
        }
        _ if key == app.user_config.keys.virustotal => {
            if app.client_config.keys.virustotal.is_empty() {
                app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::VirustotalUnloaded);
            } else if matches!(app.virustotal.status, ResultStatus::NotFound) {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::VirustotalNotFound)
            } else if matches!(app.virustotal.status, ResultStatus::NotQueried) {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::VirustotalNotQueried)
            } else {
                // Reset the menu index with switching to the view
                app.virustotal.selected_index = 0;

                // Switch to the main Virustotal view
                app.push_navigation_stack(
                    RouteId::VirustotalDetection,
                    ActiveBlock::VirustotalMenu,
                );
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
        ActiveBlock::CensysMenu => {
            censys::handler(key, app);
        }
        ActiveBlock::CensysServices => {
            censys::handler(key, app);
        }
        ActiveBlock::CensysNotFound => {
            unloaded::handler(key, app);
        }
        ActiveBlock::CensysNotQueried => {
            unloaded::handler(key, app);
        }
        ActiveBlock::CensysUnloaded => {
            unloaded::handler(key, app);
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
        ActiveBlock::VirustotalNotQueried => {
            unloaded::handler(key, app);
        }
        ActiveBlock::VirustotalNotFound => {
            unloaded::handler(key, app);
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
        ActiveBlock::ShodanNotQueried => {
            unloaded::handler(key, app);
        }
        ActiveBlock::ShodanNotFound => {
            unloaded::handler(key, app);
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
        ActiveBlock::CensysUnloaded => {
            app.pop_navigation_stack();
        }
        ActiveBlock::CensysNotQueried => {
            app.pop_navigation_stack();
        }
        ActiveBlock::CensysNotFound => {
            app.pop_navigation_stack();
        }
        ActiveBlock::ShodanUnloaded => {
            app.pop_navigation_stack();
        }
        ActiveBlock::ShodanNotFound => {
            app.pop_navigation_stack();
        }
        ActiveBlock::ShodanNotQueried => {
            app.pop_navigation_stack();
        }
        ActiveBlock::VirustotalUnloaded => {
            app.pop_navigation_stack();
        }
        ActiveBlock::VirustotalNotFound => {
            app.pop_navigation_stack();
        }
        ActiveBlock::VirustotalNotQueried => {
            app.pop_navigation_stack();
        }
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
