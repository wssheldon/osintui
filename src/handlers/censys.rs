use super::{super::app::App, common_key_events, ActiveBlock, RouteId};
use crate::{app::CENSYS_MENU, event::Key};

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            let next_index =
                common_key_events::on_down_press_handler(&CENSYS_MENU, Some(app.censys.menu_index));
            app.censys.menu_index = next_index;
            switch_view(app);
        }
        k if common_key_events::up_event(k) => {
            let next_index =
                common_key_events::on_up_press_handler(&CENSYS_MENU, Some(app.censys.menu_index));
            app.censys.menu_index = next_index;
            switch_view(app);
        }
        _ => (),
    };
}

fn switch_view(app: &mut App) {
    match app.censys.menu_index {
        0 => app.push_navigation_stack(RouteId::Censys, ActiveBlock::CensysMenu),
        1 => app.push_navigation_stack(RouteId::CensysGeoLookup, ActiveBlock::CensysMenu),
        _ => {}
    }
}
