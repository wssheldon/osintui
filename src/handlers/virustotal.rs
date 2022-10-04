use super::{
    super::app::{ActiveBlock, App, RouteId, VIRUSTOTAL_MENU},
    common_key_events,
};
use crate::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::right_event(k) => {
            match app.get_current_route().hovered_block {
                // If the user is currently on the `Menu` block, take them to `Summary`
                ActiveBlock::VirustotalMenu | ActiveBlock::Empty => {
                    match app.get_current_route().id {
                        RouteId::VirustotalDetection => {
                            app.set_current_route_state(
                                Some(ActiveBlock::VirustotalResults),
                                Some(ActiveBlock::VirustotalResults),
                            );
                        }
                        RouteId::VirustotalDetails => {
                            app.set_current_route_state(
                                Some(ActiveBlock::VirustotalWhois),
                                Some(ActiveBlock::VirustotalWhois),
                            );
                        }
                        RouteId::VirustotalCommunity => {
                            app.set_current_route_state(
                                Some(ActiveBlock::VirustotalComments),
                                Some(ActiveBlock::VirustotalComments),
                            );
                        }
                        _ => {}
                    }
                }
                _ => {}
            };
        }
        k if common_key_events::left_event(k) => {
            match app.get_current_route().hovered_block {
                ActiveBlock::VirustotalMenu | ActiveBlock::Empty => {}
                // If the user is currently on the `Results` block, take them to `Menu`
                ActiveBlock::VirustotalResults => match app.get_current_route().id {
                    RouteId::VirustotalDetection => {
                        app.set_current_route_state(
                            Some(ActiveBlock::VirustotalMenu),
                            Some(ActiveBlock::VirustotalMenu),
                        );
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        k if common_key_events::down_event(k) => {
            let next_index = common_key_events::on_down_press_handler(
                &VIRUSTOTAL_MENU,
                Some(app.virustotal.selected_index),
            );
            app.virustotal.selected_index = next_index;
            switch_view(app);
        }
        k if common_key_events::up_event(k) => {
            let next_index = common_key_events::on_up_press_handler(
                &VIRUSTOTAL_MENU,
                Some(app.virustotal.selected_index),
            );
            app.virustotal.selected_index = next_index;
            switch_view(app);
        }
        _ => (),
    };
}

fn switch_view(app: &mut App) {
    match app.virustotal.selected_index {
        0 => app.push_navigation_stack(RouteId::VirustotalDetection, ActiveBlock::VirustotalMenu),
        1 => app.push_navigation_stack(RouteId::VirustotalDetails, ActiveBlock::VirustotalMenu),
        2 => app.push_navigation_stack(RouteId::VirustotalCommunity, ActiveBlock::VirustotalMenu),
        _ => {}
    }
}
