use super::super::app::{ActiveBlock, App};
use crate::ui::{draw_table, util::get_percentage_width, TableHeader, TableHeaderItem, TableItem};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::canvas::{Canvas, Map, MapResolution},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
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
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_summary(f, app, chunks[0]);
    draw_services(f, app, chunks[1]);
    draw_map(f, app, chunks[2]);
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
                ..Default::default()
            },
            TableHeaderItem {
                text: "Transport",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "Service",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::ShodanServices,
        current_route.hovered_block == ActiveBlock::ShodanServices,
    );

    let items = app
        .shodan
        .search_ip_items
        .data
        .as_ref()
        .unwrap()
        .iter()
        .map(|services| TableItem {
            format: vec![
                services.port.to_string(),
                match services.transport.to_owned() {
                    Some(transport) => transport,
                    None => "N/A".to_string()
                },
                match services.product.to_owned() {
                    Some(product) => product,
                    None => "N/A".to_string()
                },
            ],
        })
        .collect::<Vec<TableItem>>();

    draw_table(
        f,
        app,
        layout_chunk,
        ("Services", &header),
        &items,
        app.shodan.service_index,
        highlight_state,
    );
}

pub fn draw_map<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let map = Canvas::default()
        .block(Block::default().title("World").borders(Borders::ALL))
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
