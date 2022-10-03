use super::{
    super::app::{ActiveBlock, App, RouteId},
    common_key_events,
};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => {
            if app.get_current_route().hovered_block == ActiveBlock::VirustotalSummary {
                if app.get_current_route().id == RouteId::VirustotalDetection {
                    app.set_current_route_state(
                        Some(ActiveBlock::VirustotalResults),
                        Some(ActiveBlock::VirustotalResults),
                    );
                };
            };
        }
        k if common_key_events::left_event(k) => {
            if app.get_current_route().hovered_block == ActiveBlock::VirustotalSummary {
                if app.get_current_route().id == RouteId::VirustotalDetection {
                    app.set_current_route_state(
                        Some(ActiveBlock::VirustotalMenu),
                        Some(ActiveBlock::VirustotalMenu),
                    );
                };
            };
        }
        _ => (),
    };
}
