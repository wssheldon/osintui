use super::{
    super::app::{ActiveBlock, App},
    common_key_events,
};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    // TODO: Add helper function to app
    let results: Vec<String> = app
        .virustotal
        .ip_whois_items
        .data
        .attributes
        .last_analysis_results
        .clone()
        .into_iter()
        .map(|(_id, score)| score.engine_name)
        .collect();

    match key {
        k if common_key_events::right_event(k) => {}
        k if common_key_events::left_event(k) => {
            match app.get_current_route().hovered_block {
                ActiveBlock::VirustotalResults => {
                    app.set_current_route_state(
                        Some(ActiveBlock::VirustotalSummary),
                        Some(ActiveBlock::VirustotalSummary),
                    );
                }
                _ => {}
            };
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &results,
                Some(app.virustotal.analysis_result_index),
            );
            app.virustotal.analysis_result_index = next_index;
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &results,
                Some(app.virustotal.analysis_result_index),
            );
            app.virustotal.analysis_result_index = next_index;
        }
        _ => {}
    }
}
