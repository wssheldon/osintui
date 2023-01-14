mod app;
mod banner;
mod clients;
mod config;
mod event;
mod handlers;
mod network;
mod ui;
mod user_config;

use backtrace::Backtrace;
use crossterm::{
    cursor::MoveTo,
    event::DisableMouseCapture,
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use std::{
    panic::{self, PanicInfo},
    sync::Arc,
};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

use crate::event::Key;
use app::{ActiveBlock, App, RouteId};
use clients::{censys, shodan, virustotal};
use config::Config;
use network::{IoEvent, Network};
use user_config::UserConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));

    let user_config = UserConfig::new();
    let mut client_config = Config::new();
    client_config.load_config()?;

    let censys = censys::Client::new(
        client_config.keys.censys_id.clone(),
        client_config.keys.censys_secret.clone(),
    );
    let shodan = shodan::Client::new(client_config.keys.shodan.clone());
    let virustotal = virustotal::Client::new(client_config.keys.virustotal.clone());

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    // Initialise app state
    let app = Arc::new(Mutex::new(App::new(
        sync_io_tx,
        user_config.clone(),
        client_config.clone(),
    )));

    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
        let mut network = Network::new(censys, shodan, virustotal, client_config, &app);
        start_tokio(sync_io_rx, &mut network);
    });
    start_ui(&cloned_app).await?;

    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode().expect("can run in raw mode");

    // set up a mpsc (multiproducer, single consumer) channel to communicate between
    // the input handler and the rendering loop.
    let events = event::Events::new(100);

    // Every application using tui should start by instantiating a Terminal.
    // It is a light abstraction over available backends that provides basic
    // functionalities such as clearing the screen, hiding the cursor, etc.
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        let mut app = app.lock().await;

        let current_route = app.get_current_route();
        terminal.draw(|f| match current_route.active_block {
            ActiveBlock::Error => {
                ui::draw_error_screen(f, &app);
            }
            _ => {
                ui::draw_main_layout(f, &app);
            }
        })?;

        if current_route.active_block == ActiveBlock::Input {
            terminal.show_cursor()?;
        } else {
            terminal.hide_cursor()?;
        }

        let cursor_offset = 3;

        // Put the cursor back inside the input box
        terminal.backend_mut().execute(MoveTo(
            cursor_offset + app.input_cursor_position,
            cursor_offset,
        ))?;

        match events.next()? {
            event::Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }

                let current_active_block = app.get_current_route().active_block;

                // To avoid swallowing the global key presses `q` and `-` make a special
                // case for the input handler
                if current_active_block == ActiveBlock::Input {
                    handlers::input_handler(key, &mut app);
                } else if key == app.user_config.keys.back {
                    if app.get_current_route().active_block != ActiveBlock::Input {
                        // Go back through navigation stack when not in search input mode and
                        // exit the app if there are no more places to back to
                        let pop_result = match app.pop_navigation_stack() {
                            Some(ref x) if x.id == RouteId::Search => app.pop_navigation_stack(),
                            Some(x) => Some(x),
                            None => None,
                        };
                        if pop_result.is_none() {
                            break; // Exit application
                        }
                    }
                } else {
                    handlers::handle_app(key, &mut app);
                }
            }
            event::Event::Tick => {}
        }
    }

    terminal.show_cursor()?;
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

#[tokio::main]
async fn start_tokio(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}

fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        disable_raw_mode().unwrap();
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            )),
            DisableMouseCapture
        )
        .unwrap();
    }
}
