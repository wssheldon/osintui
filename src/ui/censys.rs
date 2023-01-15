use super::super::app::{ActiveBlock, App, CENSYS_MENU};
use crate::ui::{
    draw_map, draw_selectable_list, draw_table, util::get_percentage_width, TableHeader,
    TableHeaderItem, TableItem,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw_censys<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
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

    draw_censys_menu(f, app, chunks[0]);
    draw_summary(f, app, chunks[1]);
    draw_services(f, app, chunks[2]);
}

pub fn draw_censys_menu<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::CensysMenu,
        current_route.hovered_block == ActiveBlock::CensysMenu,
    );

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "Menu",
        &CENSYS_MENU,
        highlight_state,
        Some(app.censys.menu_index),
    );
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
        current_route.active_block == ActiveBlock::CensysServices,
        current_route.hovered_block == ActiveBlock::CensysServices,
    );

    let items = &app
        .censys
        .search_ip_items
        .result
        .services
        .iter()
        .map(|services| TableItem {
            format: vec![
                match &services.port {
                    Some(port) => port.to_string(),
                    None => "N/A".to_string(),
                },
                match &services.transport_protocol {
                    Some(transport_protocol) => transport_protocol.to_string(),
                    None => "N/A".to_string(),
                },
                match &services.service_name {
                    Some(service_name) => service_name.to_string(),
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
        app.censys.service_index,
        highlight_state,
    );
}

pub fn draw_summary<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let censys_item = &app.censys.search_ip_items;
    let summary_vec = censys_item.summary_to_vec();

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
        .widths(&[Constraint::Length(20), Constraint::Percentage(100)]);

    f.render_widget(summary, layout_chunk);
}

pub fn draw_geo_info<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let items = &app.censys.search_ip_items;

    let text = vec![
        Spans::from(Span::styled(
            format!(
                "Lat: {}",
                match &items.result.location.coordinates {
                    Some(code) => code.latitude.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Lon: {}",
                match &items.result.location.coordinates {
                    Some(code) => code.longitude.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Continent: {}",
                match &items.result.location.continent {
                    Some(continent) => continent.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Continent: {}",
                match &items.result.location.country {
                    Some(country) => country.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Postal Code: {}",
                match &items.result.location.postal_code {
                    Some(postal_code) => postal_code.to_string(),
                    None => "N/A".to_string(),
                }
            ),
            Style::default().fg(app.user_config.theme.inactive),
        )),
        Spans::from(Span::styled(
            format!(
                "Timezone: {}",
                match &items.result.location.timezone {
                    Some(timezone) => timezone.to_string(),
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

pub fn draw_censys_geo_lookup<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
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

    draw_censys_menu(f, app, chunks[0]);
    draw_geo_info(f, app, chunks[1]);
    if let Some(coordinates) = &app.censys.search_ip_items.result.location.coordinates {
        draw_map(f, coordinates.latitude, coordinates.longitude, chunks[2]);
    }
}
