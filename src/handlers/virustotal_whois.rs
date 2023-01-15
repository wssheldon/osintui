use super::{
    super::app::{ActiveBlock, App, RouteId},
    common_key_events,
};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    let whois = app.virustotal.ip_whois_items.data.attributes.whois.clone();
    let results: String = match whois {
        Some(items) => items,
        None => "No Whois data found.".to_string(),
    };

    let results = results.split('\n').collect::<Vec<&str>>();

    match key {
        k if common_key_events::right_event(k) => {
            if app.get_current_route().hovered_block == ActiveBlock::VirustotalMenu
                && app.get_current_route().id == RouteId::VirustotalDetails
            {
                app.set_current_route_state(
                    Some(ActiveBlock::VirustotalWhois),
                    Some(ActiveBlock::VirustotalWhois),
                );
            };
        }
        k if common_key_events::left_event(k) => {
            if app.get_current_route().hovered_block == ActiveBlock::VirustotalWhois
                && app.get_current_route().id == RouteId::VirustotalDetails
            {
                app.set_current_route_state(
                    Some(ActiveBlock::VirustotalMenu),
                    Some(ActiveBlock::VirustotalMenu),
                );
            };
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &results,
                Some(app.virustotal.whois_result_index),
            );
            app.virustotal.whois_result_index = next_index;
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &results,
                Some(app.virustotal.whois_result_index),
            );
            app.virustotal.whois_result_index = next_index;
        }
        _ => {}
    }
}
