use super::super::app::{ActiveBlock, App, SHODAN_MENU};
use crate::ui::{
    draw_selectable_list, draw_table, util::get_percentage_width, TableHeader, TableHeaderItem,
    TableItem,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Map, MapResolution},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw_shodan<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    // Check for the width and change the contraints accordingly
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_shodan_menu(f, app, chunks[0]);
    draw_summary(f, app, chunks[1]);
    draw_services(f, app, chunks[2]);
}

pub fn draw_shodan_menu<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::ShodanMenu,
        current_route.hovered_block == ActiveBlock::ShodanMenu,
    );

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "Menu",
        &SHODAN_MENU,
        highlight_state,
        Some(app.shodan.menu_index),
    );
}

pub fn draw_summary<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let shodan_items = &app.shodan.search_ip_items;
    let summary_vec = shodan_items.summary_to_vec();

    let rows = summary_vec.iter().map(|i| {
        let cells = i.iter().map(|c| {
            let x = c.clone();
            Cell::from(x)
        });
        Row::new(cells)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .bottom_margin(1)
    });

    let summary = Table::new(rows)
        .header(Row::new(vec!["", ""]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("🌐 General Information")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Length(20), Constraint::Length(20)]);

    f.render_widget(summary, layout_chunk);
}

pub fn draw_services<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        items: vec![
            TableHeaderItem {
                text: "Port",
                width: get_percentage_width(layout_chunk.width, 0.3),
            },
            TableHeaderItem {
                text: "Transport",
                width: get_percentage_width(layout_chunk.width, 0.3),
            },
            TableHeaderItem {
                text: "Service",
                width: get_percentage_width(layout_chunk.width, 0.3),
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::ShodanServices,
        current_route.hovered_block == ActiveBlock::ShodanServices,
    );

    let items = &app
        .shodan
        .search_ip_items
        .data
        .as_ref()
        .unwrap()
        .iter()
        .map(|services| TableItem {
            format: vec![
                services.port.to_string(),
                match &services.transport {
                    Some(transport) => transport.to_string(),
                    None => "N/A".to_string(),
                },
                match &services.product {
                    Some(product) => product.to_string(),
                    None => "N/A".to_string(),
                },
            ],
        })
        .collect::<Vec<TableItem>>();

    draw_table(
        f,
        app,
        layout_chunk,
        ("Services", &header),
        items,
        app.shodan.service_index,
        highlight_state,
    );
}

pub fn draw_geo_info<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let items = &app.shodan.search_ip_items;

    let text = vec![
        Spans::from(Span::styled(
            format!("Lat: {}", &items.latitude),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!("Lon: {}", &items.longitude),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "City: {}",
                match &items.city {
                    Some(city) => city.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "County: {}",
                match &items.country_name {
                    Some(country) => country.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Code: {}",
                match &items.country_code {
                    Some(code) => code.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(app.user_config.theme.text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Geo Info")
                .border_type(BorderType::Plain),
        );

    f.render_widget(paragraph, layout_chunk);
}

pub fn draw_map<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
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
            ctx.print(
                app.shodan.search_ip_items.longitude,
                app.shodan.search_ip_items.latitude,
                "X",
                Color::Red,
            );
        })
        .marker(symbols::Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);

    f.render_widget(map, layout_chunk);
}

pub fn draw_shodan_geo_lookup<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_shodan_menu(f, app, chunks[0]);
    draw_geo_info(f, app, chunks[1]);
    draw_map(f, app, chunks[2]);
}
