use core::fmt;

use crate::app::{App, MCOrder, Page};
use crate::parameter::ParameterType;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Chart, Dataset, Gauge, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let x_size = app.ising.size as u16 / 2;
    let y_size = app.ising.size as u16 / 4;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(y_size + 1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    //
    // Title
    //

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
   
    let title = Paragraph::new(Text::styled(
        "Ising Model",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(title_block);

    //
    // Simulation
    //

    let sim_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(x_size + 4), Constraint::Min(1)])
        .split(chunks[1]);

    let ising_block = Block::default()
        // .borders(Borders::ALL)
        .style(Style::default());

    let ising_canvas = Paragraph::new(app.ising.lattice_as_braille())
        .block(ising_block)
        .alignment(Alignment::Center);

    //
    // Controls
    //

    let controls_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(1),
        ])
        .split(sim_chunks[1]);

    let mut temp_control = Gauge::default()
        .block(
            Block::default()
                .style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .title(" Temperature (T) "),
        )
        .gauge_style(Style::default().bg(Color::DarkGray).fg(Color::Red))
        .percent((app.temp_param.normalized() * 100.0) as u16)
        .label(format!("{:.3}", app.ising.temperature));

    let mut coupling_control = Gauge::default()
        .block(
            Block::default()
                .style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .title(" Coupling constant (J) "),
        )
        .gauge_style(Style::default().bg(Color::DarkGray).fg(Color::LightGreen))
        .percent((app.coupling_param.normalized() * 100.0) as u16)
        .label(format!("{:.3}", app.ising.coupling_constant));

    let mut mag_moment_control = Gauge::default()
        .block(
            Block::default()
                .style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .title(" Magnetic moment (μ) "),
        )
        .gauge_style(Style::default().bg(Color::DarkGray).fg(Color::LightMagenta))
        .percent((app.mag_moment_param.normalized() * 100.0) as u16)
        .label(format!("{:.3}", app.ising.magnetic_moment));

    let mut mag_field_strength_control = Gauge::default()
        .block(
            Block::default()
                .style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .title(" Magnetic field strength (B) "),
        )
        .gauge_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan))
        .percent((app.mag_field_strength_param.normalized() * 100.0) as u16)
        .label(format!("{:.3}", app.ising.magnetic_field_strength));

    let selected_block = Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Yellow));
    match app.current_parameter {
        ParameterType::Temp => temp_control = temp_control.block(selected_block.title(" Temperature (T) ")),
        ParameterType::Coupling => coupling_control = coupling_control.block(selected_block.title(" Coupling constant (J) ")),
        ParameterType::MagMoment => mag_moment_control = mag_moment_control.block(selected_block.title(" Magnetic moment (μ) ")),
        ParameterType::MagFieldStrength => mag_field_strength_control = mag_field_strength_control.block(selected_block.title(" Magnetic field strength (B) ")),
    };

    //
    // Magnetization and Energy charts
    //

    // let magnetization_chart = Chart::new(vec![Dataset::default().data(&[(0.0, 0.0), (10.0, 10.0)])]).block(Block::default().title("test"));

    //
    // Footer
    //

    let footer_block = Block::default().style(Style::default().bg(Color::White));

    let footer_page = match app.page {
        Page::Main => Paragraph::new(Text::styled(
            "  Quit: (q), Pause: <space>",
            Style::default().fg(Color::DarkGray),
        )),
        Page::Exit => Paragraph::new(Text::styled(
            "  Are you sure you want to quit? Yes: (y)/(q), No: (n)",
            Style::default().fg(Color::White),
        )),
    }
    .alignment(Alignment::Left)
    .block(footer_block.clone());

    let footer_spacing = Paragraph::new(Text::from("")).block(footer_block.clone());

    let footer_info_label = Paragraph::new(Text::from("Monte Carlo Order (+) / (-) :"))
        .alignment(Alignment::Right)
        .block(footer_block.clone());

    let footer_info = match app.mc_order {
        MCOrder::Linear => {
            Paragraph::new(Text::styled("O(n)", Style::default().fg(Color::DarkGray)))
        }
        MCOrder::Linearithmic => Paragraph::new(Text::styled(
            "O(n*ln(n))",
            Style::default().fg(Color::White),
        )),
        MCOrder::Quadratic => {
            Paragraph::new(Text::styled("O(n^2)", Style::default().fg(Color::White)))
        }
    }
    .alignment(Alignment::Center)
    .block(footer_block);

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(55),
            Constraint::Min(2),
            Constraint::Length(30),
            Constraint::Length(14),
        ])
        .split(chunks[3]);

    frame.render_widget(title, chunks[0]);
    frame.render_widget(ising_canvas, sim_chunks[0]);
    frame.render_widget(temp_control, controls_chunk[0]);
    frame.render_widget(coupling_control, controls_chunk[1]);
    frame.render_widget(mag_moment_control, controls_chunk[2]);
    frame.render_widget(mag_field_strength_control, controls_chunk[3]);
    // frame.render_widget(magnetization_chart, controls_chunk[4]);
    frame.render_widget(footer_page, footer_chunk[0]);
    frame.render_widget(footer_spacing, footer_chunk[1]);
    frame.render_widget(footer_info_label, footer_chunk[2]);
    frame.render_widget(footer_info, footer_chunk[3]);
}
