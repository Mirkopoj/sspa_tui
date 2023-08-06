use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::{color::ColorTrait, launcher::Launcher};
use crate::sspa::{bits, SSPAState};
use crate::state::StateKeeper;

#[derive(Clone, Copy, PartialEq)]
pub enum WidgetId {
    Registers,
    HardReset,
    Ext,
    ExtPresets,
    Compile,
    Dac,
    Offsets,
    Control,
}

pub fn layout_init<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let top_bar = chunks[0];
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Length(40),
                Constraint::Length(40),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let left_col = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(10),
                Constraint::Length(40),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(chunks[0]);
    let ext_col = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(14),
                Constraint::Min(0),
                Constraint::Length(13),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let right_col = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(22), Constraint::Min(0)].as_ref())
        .split(chunks[2]);
    let control_row = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Length(40),
                Constraint::Length(42),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(right_col[0]);
    let dac_col = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(12), Constraint::Length(10)].as_ref())
        .split(control_row[0]);
    let mut ret = Vec::new();
    ret.push(top_bar);
    ret.extend(left_col);
    ret.extend(ext_col);
    ret.extend(dac_col);
    ret.extend(&control_row[1..]);
    ret.extend(&right_col[1..]);
    ret
}

fn status<B: Backend>(chunk: Rect, f: &mut Frame<B>, status: &mut StateKeeper) {
    let reg = status.status_register();
    let values = bits(&reg);
    let block = Table::new(vec![Row::new(vec![
        Cell::from("SSPA_Active").style(Style::default().fg(values[0].color())),
        Cell::from("HW_Reflected_Power").style(Style::default().fg(values[1].color())),
        Cell::from("HW_Over_temperature").style(Style::default().fg(values[2].color())),
        Cell::from("HW_Over_drive").style(Style::default().fg(values[3].color())),
        Cell::from("HW_Gan1").style(Style::default().fg(values[4].color())),
        Cell::from("HW_Gan2").style(Style::default().fg(values[5].color())),
        Cell::from("HW_Gan3").style(Style::default().fg(values[6].color())),
        Cell::from("HW_Gan4").style(Style::default().fg(values[7].color())),
        Cell::from("SW_Reflected_Power").style(Style::default().fg(values[8].color())),
        Cell::from("SW_Direct_Power").style(Style::default().fg(values[9].color())),
        Cell::from("SW_Under_drive").style(Style::default().fg(values[10].color())),
        Cell::from("SW_Over_drive").style(Style::default().fg(values[11].color())),
        Cell::from("SW_Duty_Cycle").style(Style::default().fg(values[12].color())),
        Cell::from("SW_Over_Temperature").style(Style::default().fg(values[13].color())),
        Cell::from("SW_Over_Current").style(Style::default().fg(values[14].color())),
    ])])
    .style(Style::default().fg(reg.color()))
    .block(Block::default().title("Status").borders(Borders::ALL))
    .widths(&[
        Constraint::Length(11),
        Constraint::Length(18),
        Constraint::Length(19),
        Constraint::Length(13),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(7),
        Constraint::Length(18),
        Constraint::Length(15),
        Constraint::Length(14),
        Constraint::Length(13),
        Constraint::Length(13),
        Constraint::Length(19),
        Constraint::Length(15),
    ])
    .column_spacing(4);
    f.render_widget(block, chunk);
}

fn adc_measurements<B: Backend>(chunk: Rect, f: &mut Frame<B>, status: &mut StateKeeper) {
    let regs = status.adc_measurements();
    let items = [
        ListItem::new(format!("{:<15}:{:>20}", "Output Power", regs[0].value()))
            .style(Style::default().fg(regs[0].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Reflected Power", regs[1].value()))
            .style(Style::default().fg(regs[1].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Drive Level", regs[2].value()))
            .style(Style::default().fg(regs[2].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Temperature", regs[3].value()))
            .style(Style::default().fg(regs[3].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 1 Current", regs[4].value()))
            .style(Style::default().fg(regs[4].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 2 Current", regs[5].value()))
            .style(Style::default().fg(regs[5].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 3 Current", regs[6].value()))
            .style(Style::default().fg(regs[6].color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 4 Current", regs[7].value()))
            .style(Style::default().fg(regs[7].color())),
    ];
    let block = List::new(items)
        .block(
            Block::default()
                .title("ADC Measurements")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn registers<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let regs = state.thresholds();
    let items = [
        ListItem::new(format!(
            "Over Temperature Threshold\n{:^30}\n\n",
            regs[0].value()
        ))
        .style(Style::default().fg(regs[0].color())),
        ListItem::new(format!(
            "Temperature Threshold Hysteresis\n{:^30}\n\n",
            regs[1].value()
        ))
        .style(Style::default().fg(regs[1].color())),
        ListItem::new(format!(
            "Over Current Threshold\n{:^30}\n\n",
            regs[2].value()
        ))
        .style(Style::default().fg(regs[2].color())),
        ListItem::new(format!(
            "Duty Cylce protection Threshold\n{:^30}\n\n",
            regs[3].value()
        ))
        .style(Style::default().fg(regs[3].color())),
        ListItem::new(format!(
            "Pulse Length protection Threshold\n{:^30}\n\n",
            regs[4].value()
        ))
        .style(Style::default().fg(regs[4].color())),
        ListItem::new(format!(
            "Over Drive protection Threshold\n{:^30}\n\n",
            regs[5].value()
        ))
        .style(Style::default().fg(regs[5].color())),
        ListItem::new(format!(
            "Under Drive alarm Threshold\n{:^30}\n\n",
            regs[6].value()
        ))
        .style(Style::default().fg(regs[6].color())),
        ListItem::new(format!(
            "Output Power protection Threshold\n{:^30}\n\n",
            regs[7].value()
        ))
        .style(Style::default().fg(regs[7].color())),
        ListItem::new(format!(
            "Reflected Power protection Threshold\n{:^30}\n\n",
            regs[8].value()
        ))
        .style(Style::default().fg(regs[8].color())),
        ListItem::new(format!("SSPA serial number\n{:^30}\n\n", regs[9].value()))
            .style(Style::default().fg(regs[9].color())),
    ];
    selectable_widget(WidgetId::Registers, "Registers", &items, state, chunk, f);
}

fn state<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let state = state.sspa_state();
    let state_str = match state {
        SSPAState::Invalid => "Invalid",
        SSPAState::Boot => "Boot",
        SSPAState::StandBy => "StandBy",
        SSPAState::Failure => "Failure",
        SSPAState::Disabled => "Disabled",
        SSPAState::Nominal => "Nominal",
        SSPAState::Warning => "Warning",
        SSPAState::ProtectionHW => "ProtectionHW",
        SSPAState::Protection => "Protection",
    };
    let text = vec![Spans::from(Span::styled(
        state_str,
        Style::default().fg(state.color()),
    ))];
    let block = Paragraph::new(text)
        .block(Block::default().title("State").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(block, chunk);
}

fn firmware_version<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let vernum = state.version_number();
    let mayor = (vernum.value() >> 8) & 0x7F;
    let minor = (vernum.value() >> 8) & 0x0F;
    let patch = (vernum.value() >> 8) & 0x0F;
    let text = vec![Spans::from(Span::styled(
        format!("v{}.{}.{}", mayor, minor, patch),
        Style::default().fg(vernum.color()),
    ))];
    let block = Paragraph::new(text)
        .block(
            Block::default()
                .title("Firmware Version")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(block, chunk);
}

fn hard_reset<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let text = vec![Spans::from(Span::styled(
        "HARD RESET",
        Style::default().fg(Color::Red),
    ))];
    let block = Paragraph::new(text)
        .block(Block::default().title("Hard Reset").borders(Borders::ALL))
        .style(
            Style::default().fg(if state.is_widget_selected(WidgetId::HardReset) {
                Color::Green
            } else {
                Color::White
            }),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(block, chunk);
}

fn ext_signals<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let (powen, current, values) = state.ext_signals();
    let items = [
        ListItem::new("Power Enable").style(Style::default().fg(powen.color())),
        ListItem::new(format!("TnR:\n{:>12}:{:?}", "Current", current)),
        ListItem::new(format!("{:>12}:{:>20}", "Period", values[0])),
        ListItem::new(format!("{:>12}:{:>20}", "Pulse Width", values[1])),
        ListItem::new(format!("{:>12}:{:>20}", "Count", values[2])),
        ListItem::new(format!("\n{:^38}", "[LAUNCH]")),
        ListItem::new(format!("\n{:^38}", "[STOP]")),
        ListItem::new(format!("\n{:^38}", "[SAVE]")),
    ];
    selectable_widget(WidgetId::Ext, "Ext Signals", &items, state, chunk, f);
}

fn ext_signals_presets<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let items = [];
    selectable_widget(
        WidgetId::ExtPresets,
        "Ext Signals Presets",
        &items,
        state,
        chunk,
        f,
    );
}

fn dac<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let values = state.dac();
    let items = [
        ListItem::new(format!("{:<15}:{:>20}", "Output Power", values[0])),
        ListItem::new(format!("{:<15}:{:>20}", "Reflected Power", values[1])),
        ListItem::new(format!("{:<15}:{:>20}", "Drive Level", values[2])),
        ListItem::new(format!("{:<15}:{:>20}", "Temperature", values[3])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 1 Current", values[4])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 2 Current", values[5])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 3 Current", values[6])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 4 Current", values[7])),
        ListItem::new(format!("\n{:^38}", "[CLEAR]")),
    ];
    selectable_widget(WidgetId::Dac, "DAC", &items, state, chunk, f);
}

fn offsets<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let values = state.offsets();
    let items = [
        ListItem::new(format!("{:<15}:{:>20}", "Output Power", values[0])),
        ListItem::new(format!("{:<15}:{:>20}", "Reflected Power", values[1])),
        ListItem::new(format!("{:<15}:{:>20}", "Drive Level", values[2])),
        ListItem::new(format!("{:<15}:{:>20}", "Temperature", values[3])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 1 Current", values[4])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 2 Current", values[5])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 3 Current", values[6])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 4 Current", values[7])),
    ];
    selectable_widget(WidgetId::Offsets, "Offsets", &items, state, chunk, f);
}

fn control<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let reg = state.control_register();
    let values = bits(&reg);
    let items = [
        ListItem::new(format!("\n{:^40}", "[Store to Non Volatile Memory]"))
            .style(Style::default().fg(reg.color())),
        ListItem::new(format!("\n{:^40}", "[Load from Non Volatile Memory]"))
            .style(Style::default().fg(reg.color())),
        ListItem::new(format!("\n{:^40}", "[Alarms Reset]")).style(Style::default().fg(reg.color())),
        ListItem::new(format!("\n{:^40}", "[SSPA Reset]")).style(Style::default().fg(reg.color())),
        ListItem::new(format!("\n{:^40}", "[SSPA Disable]")).style(Style::default().fg(reg.color())),
        ListItem::new(format!("\n{:^40}", "SW Reflected Power protection disable"))
            .style(Style::default().fg(values[10].color())),
        ListItem::new(format!("{:^40}", "SW Over drive protection disable"))
            .style(Style::default().fg(values[11].color())),
        ListItem::new(format!("{:^40}", "SW Duty cycle protection disable"))
            .style(Style::default().fg(values[12].color())),
        ListItem::new(format!("{:^40}", "SW Over temperature protection disable"))
            .style(Style::default().fg(values[13].color())),
        ListItem::new(format!("{:^40}", "SW Over current protection disable"))
            .style(Style::default().fg(values[14].color())),
    ];
    selectable_widget(WidgetId::Control, "Control", &items, state, chunk, f);
}

fn compile<B: Backend>(chunk: Rect, f: &mut Frame<B>, state: &mut StateKeeper) {
    let items = [
        ListItem::new(format!("\n{:^38}", "[Build]")),
        ListItem::new(format!("\n{:^38}", "[Clean Build]")),
        ListItem::new(format!("\n{:^38}", "[Clean Build and Flash]")),
        ListItem::new(format!("\n{:^38}", "[Build and Flash]")),
        ListItem::new(format!("\n{:^38}", "[Flash]")),
    ];
    selectable_widget(WidgetId::Compile, "Compile", &items, state, chunk, f);
}

fn terminal<B: Backend>(chunk: Rect, f: &mut Frame<B>, term: &mut Launcher) {
    let text = Text::from(term.read());
    let block = Paragraph::new(text)
        .block(Block::default().title("Terminal").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(block, chunk);
}

fn ssh<B: Backend>(chunk: Rect, f: &mut Frame<B>, ssh_term: &mut Launcher) {
    let text = Text::from(ssh_term.read());
    let block = Paragraph::new(text)
        .block(Block::default().title("SSH").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(block, chunk);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, state_keeper: &mut StateKeeper, term: &mut Launcher, ssh_term: &mut Launcher) {
    let chunks = layout_init(f);
    status(chunks[0], f, state_keeper);
    adc_measurements(chunks[1], f, state_keeper);
    registers(chunks[2], f, state_keeper);
    state(chunks[3], f, state_keeper);
    firmware_version(chunks[5], f, state_keeper);
    hard_reset(chunks[6], f, state_keeper);
    ext_signals(chunks[7], f, state_keeper);
    ext_signals_presets(chunks[8], f, state_keeper);
    compile(chunks[9], f, state_keeper);
    dac(chunks[10], f, state_keeper);
    offsets(chunks[11], f, state_keeper);
    control(chunks[12], f, state_keeper);
    ssh(chunks[13], f, ssh_term);
    terminal(chunks[14], f, term);
}

fn selectable_widget<B: Backend>(
    wid: WidgetId,
    title: &str,
    items: &[ListItem],
    state: &mut StateKeeper,
    chunk: Rect,
    f: &mut Frame<B>,
) {
    let block = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(Style::default().fg(if state.is_widget_selected(wid) {
            Color::Green
        } else {
            Color::White
        }))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED));
    f.render_stateful_widget(block, chunk, state.selected_item(wid));
}
