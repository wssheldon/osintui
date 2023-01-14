pub mod censys;
pub mod shodan;
pub mod util;
pub mod virustotal;

use super::{
    app::{ActiveBlock, App, RouteId},
    banner::BANNER,
};
use crate::ui::{
    censys::{draw_censys, draw_censys_geo_lookup},
    shodan::{draw_shodan, draw_shodan_geo_lookup},
    util::get_color,
    virustotal::{draw_virustotal_community, draw_virustotal_details, draw_virustotal_detection},
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans, Text},
    widgets::canvas::{Canvas, Map, MapResolution},
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame,
};

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
        }
    }
}

#[derive(PartialEq)]
pub enum ColumnId {
    None,
}

impl Default for ColumnId {
    fn default() -> Self {
        ColumnId::None
    }
}

#[derive(Default)]
pub struct TableHeaderItem<'a> {
    text: &'a str,
    width: u16,
}

pub struct TableHeader<'a> {
    items: Vec<TableHeaderItem<'a>>,
}

pub struct TableItem {
    format: Vec<String>,
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(2)].as_ref())
        .split(f.size());

    draw_menu_search_help_box(f, app, parent_layout[0]);
    draw_routes(f, app, parent_layout[1]);
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Search => {}
        RouteId::Home => {
            draw_home(f, app, chunks[0]);
        }
        RouteId::SearchResult => {
            draw_search_result_page(f, app, chunks[0]);
        }
        RouteId::Censys => {
            draw_censys(f, app, chunks[0]);
        }
        RouteId::CensysGeoLookup => {
            draw_censys_geo_lookup(f, app, chunks[0]);
        }
        RouteId::VirustotalDetection => {
            draw_virustotal_detection(f, app, chunks[0]);
        }
        RouteId::VirustotalDetails => {
            draw_virustotal_details(f, app, chunks[0]);
        }
        RouteId::VirustotalCommunity => {
            draw_virustotal_community(f, app, chunks[0]);
        }
        RouteId::Unloaded => {
            draw_unloaded(f, app, chunks[0]);
        }
        RouteId::Shodan => {
            draw_shodan(f, app, chunks[0]);
        }
        RouteId::ShodanGeoLookup => {
            draw_shodan_geo_lookup(f, app, chunks[0]);
        }
        RouteId::Error => {} // This is handled as a "full screen" route in main.rs
    };
}

pub fn draw_menu_search_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    // Check for the width and change the contraints accordingly
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
    );

    let input_string: String = app.input.iter().collect();
    let lines = Text::from((&input_string).as_str());
    let input = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled("Search", get_color(highlight_state))),
    );

    f.render_widget(input, chunks[0]);

    let menu = vec!["Home", "Shodan", "Virustotal", "Censys", "Quit"]
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let active_menu_item = MenuItem::Home;
    let tabs = Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    f.render_widget(tabs, chunks[1]);

    let help_block_text = if app.is_loading {
        (app.user_config.theme.hint, "Loading...")
    } else if app.is_input_error {
        (app.user_config.theme.hint, "ERR: Not valid.")
    } else {
        (app.user_config.theme.inactive, "Waiting for input...")
    };

    let block = Block::default()
        .title(Span::styled(
            "Status",
            Style::default().fg(help_block_text.0),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(help_block_text.0));

    let lines = Text::from(help_block_text.1);
    let help = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(help_block_text.0));
    f.render_widget(help, chunks[2]);
}

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    // Check for the width and change the contraints accordingly
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(layout_chunk);

    draw_welcome_page(f, app, chunks[0]);
    draw_integrations(f, app, chunks[1]);
}

pub fn draw_welcome_page<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    // Check for the width and change the contraints accordingly
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    let welcome = Block::default()
        .title(Span::styled("Welcome!", get_color(highlight_state)))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state));
    f.render_widget(welcome, layout_chunk);

    // Banner text with correct styling
    let mut top_text = Text::from(BANNER);
    top_text.patch_style(Style::default().fg(app.user_config.theme.banner));

    // // Contains the banner
    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(Color::LightRed))
        .alignment(Alignment::Center)
        .block(Block::default());

    f.render_widget(top_text, chunks[0]);

    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("'/' to search")]),
        Spans::from(vec![Span::raw("'s' to access shodan")]),
        Spans::from(vec![Span::raw("'v' to access virustotal")]),
    ])
    .style(Style::default().fg(app.user_config.theme.text))
    .alignment(Alignment::Center)
    .block(Block::default())
    .wrap(Wrap { trim: false });

    f.render_widget(home, chunks[1]);
}

pub fn draw_integrations<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let api_view = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            " {} Shodan",
            match app.client_config.keys.shodan.is_empty() {
                true => "❌",
                false => "✅",
            }
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            " {} Virustotal",
            match app.client_config.keys.virustotal.is_empty() {
                true => "❌",
                false => "✅",
            }
        ))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            " {} Censys",
            match app.client_config.keys.censys_id.is_empty()
                && app.client_config.keys.censys_secret.is_empty()
            {
                true => "❌",
                false => "✅",
            }
        ))]),
    ])
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Integrations")
            .border_type(BorderType::Plain),
    );
    f.render_widget(api_view, layout_chunk);
}

pub fn draw_search_result_page<B>(f: &mut Frame<B>, _app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Lookup complete!")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("'/' to search")]),
        Spans::from(vec![Span::raw("'s' to access shodan")]),
        Spans::from(vec![Span::raw("'v' to access virustotal")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    f.render_widget(home, layout_chunk);
}

pub fn draw_error_screen<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let error_text = vec![
        Spans::from(vec![
            Span::raw("Api response: "),
            Span::styled(
                &app.api_error,
                Style::default().fg(app.user_config.theme.error_text),
            ),
        ]),
        Spans::from(Span::styled(
            "\nPress <Esc> to return",
            Style::default().fg(app.user_config.theme.inactive),
        )),
    ];

    let error_paragraph = Paragraph::new(error_text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(app.user_config.theme.text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Error",
                    Style::default().fg(app.user_config.theme.error_border),
                ))
                .border_style(Style::default().fg(app.user_config.theme.error_border)),
        );
    f.render_widget(error_paragraph, chunks[0]);
}

fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    title: &str,
    items: &[S],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: std::convert::AsRef<str>,
{
    let mut state = ListState::default();
    state.select(selected_index);

    let items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(Span::raw(i.as_ref())))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(Span::styled(title, get_color(highlight_state)))
                .borders(Borders::ALL)
                .border_style(get_color(highlight_state)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .highlight_style(get_color(highlight_state).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, layout_chunk, &mut state);
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &TableHeader), // (title, header colums)
    items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    let selected_style = get_color(highlight_state).add_modifier(Modifier::BOLD);

    let (title, header) = table_layout;

    // Make sure that the selected item is visible on the page. Need to add some rows of padding
    // to chunk height for header and header space to get a true table height
    let padding = 5;
    let offset = layout_chunk
        .height
        .checked_sub(padding)
        .and_then(|height| selected_index.checked_sub(height as usize))
        .unwrap_or(0);

    let rows = items.iter().skip(offset).enumerate().map(|(i, item)| {
        let formatted_row = item.format.clone();
        let mut style = Style::default().fg(app.user_config.theme.text);

        // Next check if the item is under selection.
        if Some(i) == selected_index.checked_sub(offset) {
            style = selected_style;
        }

        // Return row styled data
        Row::new(formatted_row).style(style)
    });

    let widths = header
        .items
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect::<Vec<tui::layout::Constraint>>();

    let table = Table::new(rows)
        .header(
            Row::new(header.items.iter().map(|h| h.text))
                .style(Style::default().fg(app.user_config.theme.header)),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(app.user_config.theme.text))
                .title(Span::styled(title, get_color(highlight_state)))
                .border_style(get_color(highlight_state)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .widths(&widths);

    f.render_widget(table, layout_chunk);
}

pub fn draw_map<B>(f: &mut Frame<B>, lat: f64, long: f64, layout_chunk: Rect)
where
    B: Backend,
{
    let map = Canvas::default()
        .block(Block::default().title("Geo Lookup").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.layer();
            ctx.print(long, lat, "X", Color::Red);
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);

    f.render_widget(map, layout_chunk);
}

pub fn draw_unloaded<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let plugin = match app.get_current_route().active_block {
        ActiveBlock::VirustotalUnloaded => "Virustotal",
        ActiveBlock::ShodanUnloaded => "Shodan",
        ActiveBlock::CensysUnloaded => "Censys",
        _ => "",
    };

    let text = vec![
        Spans::from(Span::styled(
            format!("\nThe {} plugin is not currently loaded.", plugin),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            "\nPress <Esc> to return",
            Style::default().fg(app.user_config.theme.inactive),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(app.user_config.theme.text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "Error",
                    Style::default().fg(app.user_config.theme.error_border),
                ))
                .border_style(Style::default().fg(app.user_config.theme.error_border)),
        );
    f.render_widget(paragraph, layout_chunk);
}
