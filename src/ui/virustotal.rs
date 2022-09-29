use super::super::app::{ActiveBlock, App};
use crate::clients::virustotal::AnalysisResult;
use crate::ui::util::{get_color, get_percentage_width};
use crate::ui::{draw_selectable_list, draw_table, TableHeader, TableHeaderItem, TableItem};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn draw_virustotal_menu<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::VirustotalMenu,
        current_route.hovered_block == ActiveBlock::VirustotalMenu,
    );

    let menu_list = ["Detection", "Details", "Relations", "Community"];

    draw_selectable_list(
        f,
        app,
        layout_chunk,
        "Menu",
        &menu_list,
        highlight_state,
        Some(app.virustotal.selected_index),
    );
}

pub fn draw_virustotal_detection<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_virustotal_menu(f, app, chunks[0]);
    draw_virustotal_detection_summary(f, app, chunks[1]);
    draw_virustotal_detection_results(f, app, chunks[2]);
}

pub fn draw_virustotal_detection_summary<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = &app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::VirustotalSummary,
        current_route.hovered_block == ActiveBlock::VirustotalSummary,
    );

    let vt_resp = app.virustotal.ip_whois_items.to_owned();
    let malicious = &vt_resp.data.attributes.last_analysis_stats.malicious;
    let suspicious = &vt_resp.data.attributes.last_analysis_stats.suspicious;
    let network = &vt_resp.data.attributes.network;
    let votes = &vt_resp.data.attributes.total_votes;
    let asn = &vt_resp.data.attributes.asn.to_string();
    let as_owner = &vt_resp.data.attributes.as_owner;
    let total_malicious = malicious + suspicious;
    let ip = &vt_resp.data.id;

    let summary_color = if total_malicious > 0 {
        Color::LightRed
    } else {
        Color::LightGreen
    };

    let summary = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            format!(
                "{} security vendors flagged this IP address as malicious",
                total_malicious
            ),
            Style::default()
                .fg(summary_color)
                .add_modifier(Modifier::BOLD),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!("{} ( {} )", ip, network))]),
        Spans::from(vec![Span::raw(format!("{} ( {} )", asn, as_owner))]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(format!(
            "❌ {} --------------------- {} ✅",
            votes.malicious, votes.harmless
        ))]),
        Spans::from(vec![Span::styled(
            String::from("Community Score"),
            Style::default().add_modifier(Modifier::DIM),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(Span::styled("Summary", get_color(highlight_state)))
            .border_type(BorderType::Plain),
    );

    f.render_widget(summary, layout_chunk)
}

pub fn draw_virustotal_detection_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        items: vec![
            TableHeaderItem {
                text: "Engine",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
            TableHeaderItem {
                text: "Result",
                width: get_percentage_width(layout_chunk.width, 0.3),
                ..Default::default()
            },
        ],
    };

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::VirustotalResults,
        current_route.hovered_block == ActiveBlock::VirustotalResults,
    );

    let mut results: Vec<AnalysisResult> = Vec::new();

    let tmp_res: Vec<_> = app
        .virustotal
        .ip_whois_items
        .data
        .attributes
        .last_analysis_results
        .values()
        .collect();

    for pair in tmp_res.iter() {
        if !pair.result.contains("clean") && !pair.result.contains("unrated") {
            results.insert(0, pair.to_owned().clone());
        } else {
            results.push(pair.to_owned().clone())
        }
    }

    let items = results
        .iter()
        .map(|scan| TableItem {
            format: vec![
                scan.engine_name.to_owned(),
                match scan.result.as_str() {
                    "clean" => {
                        format!("✅ {}", scan.result)
                    }
                    "unrated" => {
                        format!("❔ {}", scan.result)
                    }
                    _ => {
                        format!("❗️ {}", scan.result)
                    }
                },
            ],
        })
        .collect::<Vec<TableItem>>();

    draw_table(
        f,
        app,
        layout_chunk,
        ("Scans", &header),
        &items,
        app.virustotal.analysis_result_index,
        highlight_state,
    );
}

pub fn draw_virustotal_details<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(layout_chunk);

    draw_virustotal_menu(f, app, chunks[0]);
    draw_virustotal_whois_lookup(f, app, chunks[1]);
}

pub fn draw_virustotal_whois_lookup<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = TableHeader {
        items: vec![TableHeaderItem {
            text: "",
            width: 100,
            ..Default::default()
        }],
    };

    let items = app
        .virustotal
        .ip_whois_items
        .data
        .attributes
        .whois
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|i| TableItem {
            format: vec![i.to_owned()],
        })
        .collect::<Vec<TableItem>>();

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::VirustotalWhois,
        current_route.hovered_block == ActiveBlock::VirustotalWhois,
    );

    draw_table(
        f,
        app,
        layout_chunk,
        ("Whois Lookup", &header),
        &items,
        app.virustotal.whois_result_index,
        highlight_state,
    );
}
