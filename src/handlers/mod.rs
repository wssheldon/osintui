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
    match key {
        Key::Esc => handle_escape(app),
        _ if key == app.user_config.keys.home => handle_home(app),
        _ if key == app.user_config.keys.search => handle_search(app),
        _ if key == app.user_config.keys.censys => handle_censys(app),
        _ if key == app.user_config.keys.shodan => handle_shodan(app),
        _ if key == app.user_config.keys.virustotal => handle_virustotal(app),
        _ => handle_block_events(key, app),
    }
}

fn handle_home(app: &mut App) {
    app.push_navigation_stack(RouteId::Home, ActiveBlock::Input);
}

fn handle_search(app: &mut App) {
    app.set_current_route_state(Some(ActiveBlock::Input), Some(ActiveBlock::Input));
}

fn handle_censys(app: &mut App) {
    if app.client_config.keys.censys_secret.is_empty()
        && app.client_config.keys.censys_id.is_empty()
    {
        app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::CensysUnloaded);
    } else {
        match app.censys.status {
            ResultStatus::NotFound => {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::CensysNotFound)
            }
            ResultStatus::NotQueried => {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::CensysNotQueried)
            }
            _ => {
                app.censys.menu_index = 0;
                app.push_navigation_stack(RouteId::Censys, ActiveBlock::CensysMenu);
            }
        }
    }
}

fn handle_shodan(app: &mut App) {
    if app.client_config.keys.shodan.is_empty() {
        app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::ShodanUnloaded);
    } else {
        match app.shodan.status {
            ResultStatus::NotFound => {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::ShodanNotFound)
            }
            ResultStatus::NotQueried => {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::ShodanNotQueried)
            }
            _ => {
                app.shodan.menu_index = 0;
                app.push_navigation_stack(RouteId::Shodan, ActiveBlock::ShodanMenu);
            }
        }
    }
}

fn handle_virustotal(app: &mut App) {
    if app.client_config.keys.virustotal.is_empty() {
        app.push_navigation_stack(RouteId::Unloaded, ActiveBlock::VirustotalUnloaded);
    } else {
        match app.virustotal.status {
            ResultStatus::NotFound => {
                app.push_navigation_stack(RouteId::NotFound, ActiveBlock::VirustotalNotFound)
            }
            ResultStatus::NotQueried => {
                app.push_navigation_stack(RouteId::NotQueried, ActiveBlock::VirustotalNotQueried)
            }
            _ => {
                app.virustotal.selected_index = 0;
                app.push_navigation_stack(
                    RouteId::VirustotalDetection,
                    ActiveBlock::VirustotalMenu,
                );
            }
        }
    }
}

// Handle event for the current active block
fn handle_block_events(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    let active_block = current_route.active_block;

    match active_block {
        // Input block
        ActiveBlock::Input => input::handler(key, app),

        // Home block
        ActiveBlock::Home => home::handler(key, app),

        // Empty block
        ActiveBlock::Empty => empty::handler(key, app),

        // Error block
        ActiveBlock::Error => error_screen::handler(key, app),

        // Search result block
        ActiveBlock::SearchResult => search_result::handler(key, app),

        // Censys blocks
        ActiveBlock::CensysMenu | ActiveBlock::CensysServices => censys::handler(key, app),
        ActiveBlock::CensysNotFound
        | ActiveBlock::CensysNotQueried
        | ActiveBlock::CensysUnloaded => unloaded::handler(key, app),

        // Virustotal blocks
        ActiveBlock::VirustotalMenu
        | ActiveBlock::VirustotalSummary
        | ActiveBlock::VirustotalResults
        | ActiveBlock::VirustotalWhois
        | ActiveBlock::VirustotalComments => virustotal_handler(key, app),
        ActiveBlock::VirustotalNotFound
        | ActiveBlock::VirustotalNotQueried
        | ActiveBlock::VirustotalUnloaded => unloaded::handler(key, app),

        // Shodan blocks
        ActiveBlock::ShodanMenu | ActiveBlock::ShodanServices => shodan::handler(key, app),
        ActiveBlock::ShodanNotQueried
        | ActiveBlock::ShodanNotFound
        | ActiveBlock::ShodanUnloaded => unloaded::handler(key, app),
    }
}

fn virustotal_handler(key: Key, app: &mut App) {
    let active_block = app.get_current_route().active_block;
    match active_block {
        ActiveBlock::VirustotalMenu => virustotal::handler(key, app),
        ActiveBlock::VirustotalSummary => virustotal_summary::handler(key, app),
        ActiveBlock::VirustotalResults => virustotal_results::handler(key, app),
        ActiveBlock::VirustotalWhois => virustotal_whois::handler(key, app),
        ActiveBlock::VirustotalComments => virustotal_comments::handler(key, app),
        _ => unreachable!(),
    }
}

fn handle_escape(app: &mut App) {
    let should_pop = matches!(
        app.get_current_route().active_block,
        ActiveBlock::Error
            | ActiveBlock::CensysUnloaded
            | ActiveBlock::CensysNotQueried
            | ActiveBlock::CensysNotFound
            | ActiveBlock::ShodanUnloaded
            | ActiveBlock::ShodanNotFound
            | ActiveBlock::ShodanNotQueried
            | ActiveBlock::VirustotalUnloaded
            | ActiveBlock::VirustotalNotFound
            | ActiveBlock::VirustotalNotQueried,
    );

    if should_pop {
        app.pop_navigation_stack();
    } else {
        app.set_current_route_state(Some(ActiveBlock::Empty), None);
    }
}
