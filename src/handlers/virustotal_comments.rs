use super::common_key_events;
use crate::{app::App, event::Key, ActiveBlock, RouteId};

const SMALL_SCROLL: u16 = 1;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_key_events::down_event(k) => {
            app.virustotal.comment_scroll += SMALL_SCROLL;
        }
        k if common_key_events::up_event(k) => {
            if app.virustotal.comment_scroll > 0 {
                app.virustotal.comment_scroll -= SMALL_SCROLL;
            }
        }
        k if common_key_events::left_event(k) => {
            if app.get_current_route().hovered_block == ActiveBlock::VirustotalComments
                && app.get_current_route().id == RouteId::VirustotalCommunity
            {
                app.set_current_route_state(
                    Some(ActiveBlock::VirustotalMenu),
                    Some(ActiveBlock::VirustotalMenu),
                );
            };
        }
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_small_down_press() {
        let mut app = App::default();

        handler(Key::Down, &mut app);
        assert_eq!(app.virustotal.comment_scroll, SMALL_SCROLL);

        handler(Key::Down, &mut app);
        assert_eq!(app.virustotal.comment_scroll, SMALL_SCROLL * 2);
    }

    #[test]
    fn on_small_up_press() {
        let mut app = App::default();

        handler(Key::Up, &mut app);
        assert_eq!(app.virustotal.comment_scroll, 0);

        app.virustotal.comment_scroll = 1;

        handler(Key::Up, &mut app);
        assert_eq!(app.virustotal.comment_scroll, 0);

        // Check that smashing the up button doesn't go to negative scroll (which would cause a crash)
        handler(Key::Up, &mut app);
        handler(Key::Up, &mut app);
        handler(Key::Up, &mut app);
        assert_eq!(app.virustotal.comment_scroll, 0);
    }
}
