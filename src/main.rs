use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame, Terminal,
};

fn chunks<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
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
        .constraints([Constraint::Length(13), Constraint::Min(0)].as_ref())
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
                Constraint::Length(50),
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

enum RegisterState {
    ParityError,
    Warning,
    Ok,
}

struct Register {
    state: RegisterState,
    value: u16,
}

impl Register {
    pub fn new(value: u16) -> Register {
        let parity = value.count_ones() % 2 == 0;
        let state = match (parity, value) {
            (false, _) => RegisterState::ParityError,
            (true, u16::MAX) => RegisterState::Warning,
            (true, _) => RegisterState::Ok,
        };
        Register { state, value }
    }

    pub fn state_color(&self) -> Color {
        match self.state {
            RegisterState::Ok => Color::White,
            RegisterState::Warning => Color::Yellow,
            RegisterState::ParityError => Color::Red,
        }
    }
}

fn bits(reg: &Register) -> [bool; 15] {
    (0..15)
        .rev()
        .map(|n| (reg.value & (1 << n)) != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap()
}

fn status<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let reg = Register::new(0);
    let values = bits(&reg);
    let block = Table::new(vec![Row::new(vec![
        Cell::from("SSPA_Active").style(Style::default().fg(if values[0] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Reflected_Power").style(Style::default().fg(if values[1] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Over_temperature").style(Style::default().fg(if values[2] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Over_drive").style(Style::default().fg(if values[3] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Gan1").style(Style::default().fg(if values[4] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Gan2").style(Style::default().fg(if values[5] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Gan3").style(Style::default().fg(if values[6] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("HW_Gan4").style(Style::default().fg(if values[7] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Reflected_Power").style(Style::default().fg(if values[8] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Direct_Power").style(Style::default().fg(if values[9] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Under_drive").style(Style::default().fg(if values[10] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Over_drive").style(Style::default().fg(if values[11] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Duty_Cycle").style(Style::default().fg(if values[12] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Over_Temperature").style(Style::default().fg(if values[13] {
            Color::White
        } else {
            Color::DarkGray
        })),
        Cell::from("SW_Over_Current").style(Style::default().fg(if values[14] {
            Color::White
        } else {
            Color::DarkGray
        })),
    ])])
    .style(Style::default().fg(reg.state_color()))
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

fn adc_measurements<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let regs = [
        Register::new(65534),
        Register::new(65535),
        Register::new(65532),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
    ];
    let items = [
        ListItem::new(format!("{:<15}:{:>20}", "Output Power", regs[0].value))
            .style(Style::default().fg(regs[0].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Reflected Power", regs[1].value))
            .style(Style::default().fg(regs[1].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Drive Level", regs[2].value))
            .style(Style::default().fg(regs[2].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Temperature", regs[3].value))
            .style(Style::default().fg(regs[3].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 1 Current", regs[4].value))
            .style(Style::default().fg(regs[4].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 2 Current", regs[5].value))
            .style(Style::default().fg(regs[5].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 3 Current", regs[6].value))
            .style(Style::default().fg(regs[6].state_color())),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 4 Current", regs[7].value))
            .style(Style::default().fg(regs[7].state_color())),
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

fn registers<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let regs = [
        Register::new(65534),
        Register::new(65535),
        Register::new(65532),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
        Register::new(65535),
    ];

    let items = [
        ListItem::new("Over Temperature Threshold").style(Style::default().fg(regs[0].state_color())),
        ListItem::new(format!(".   {}", regs[0].value)).style(Style::default().fg(regs[0].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[0].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[0].state_color())),
        ListItem::new("Temperature Threshold Hysteresis").style(Style::default().fg(regs[1].state_color())),
        ListItem::new(format!(".   {}", regs[1].value)).style(Style::default().fg(regs[1].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[1].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[1].state_color())),
        ListItem::new("Over Current Threshold").style(Style::default().fg(regs[2].state_color())),
        ListItem::new(format!(".   {}", regs[2].value)).style(Style::default().fg(regs[2].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[2].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[2].state_color())),
        ListItem::new("Duty Cylce protection Threshold").style(Style::default().fg(regs[3].state_color())),
        ListItem::new(format!(".   {}", regs[3].value)).style(Style::default().fg(regs[3].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[3].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[3].state_color())),
        ListItem::new("Pulse Length protection Threshold").style(Style::default().fg(regs[4].state_color())),
        ListItem::new(format!(".   {}", regs[4].value)).style(Style::default().fg(regs[4].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[4].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[4].state_color())),
        ListItem::new("Over Drive protection Threshold").style(Style::default().fg(regs[5].state_color())),
        ListItem::new(format!(".   {}", regs[5].value)).style(Style::default().fg(regs[5].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[5].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[5].state_color())),
        ListItem::new("Under Drive alarm Threshold").style(Style::default().fg(regs[6].state_color())),
        ListItem::new(format!(".   {}", regs[6].value)).style(Style::default().fg(regs[6].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[6].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[6].state_color())),
        ListItem::new("Output Power protection Threshold").style(Style::default().fg(regs[7].state_color())),
        ListItem::new(format!(".   {}", regs[7].value)).style(Style::default().fg(regs[7].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[7].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[7].state_color())),
        ListItem::new("Reflected Power protection Threshold").style(Style::default().fg(regs[8].state_color())),
        ListItem::new(format!(".   {}", regs[8].value)).style(Style::default().fg(regs[8].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[8].state_color())),
        ListItem::new(".").style(Style::default().fg(regs[8].state_color())),
        ListItem::new("SSPA serial number").style(Style::default().fg(regs[9].state_color())),
        ListItem::new(format!(".   {}", regs[9].value)).style(Style::default().fg(regs[9].state_color())),
    ];
    let block = List::new(items)
        .block(Block::default().title("Registers").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

#[allow(dead_code)]
enum State {
    Boot,
    StandBy,
    Failure,
    Disabled,
    Nominal,
    Warning,
    ProtectionHW,
    Protection,
}

fn state<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let state = State::StandBy;
    let state = match state {
        State::Boot => "Boot",
        State::StandBy => "StandBy",
        State::Failure => "Failure",
        State::Disabled => "Disabled",
        State::Nominal => "Nominal",
        State::Warning => "Warning",
        State::ProtectionHW => "ProtectionHW",
        State::Protection => "Protection",
    };
    let text = vec![Spans::from(Span::raw(state))];
    let block = Paragraph::new(text)
        .block(Block::default().title("State").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(block, chunk);
}

fn firmware_version<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let text = vec![Spans::from(Span::raw("v1.0.1"))];
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

fn hard_reset<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let text = vec![Spans::from(Span::styled(
        "HARD RESET",
        Style::default().fg(Color::Red),
    ))];
    let block = Paragraph::new(text)
        .block(Block::default().title("Hard Reset").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(block, chunk);
}

fn ext_signals<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let powen = false;
    let values = [65536, 65536, 65536];
    let items = [
        ListItem::new(format!("{}:{:>20}", "Power Enable", powen)),
        ListItem::new(format!("{}:", "TnR")),
        ListItem::new(format!("{:>12}:{:>20}", "Period", values[0])),
        ListItem::new(format!("{:>12}:{:>20}", "Pulse Width", values[1])),
        ListItem::new(format!("{:>12}:{:>20}", "Count", values[2])),
        ListItem::new("."),
        ListItem::new(format!("{:^38}", "[LAUNCH]")),
        ListItem::new("."),
        ListItem::new(format!("{:^38}", "[STOP]")),
        ListItem::new("."),
        ListItem::new(format!("{:^38}", "[SAVE]")),
    ];
    let block = List::new(items)
        .block(Block::default().title("Ext Signals").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn ext_signals_presets<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let block = Block::default()
        .title("Ext Signals Presets")
        .borders(Borders::ALL);
    f.render_widget(block, chunk);
}

fn dac<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let values = [65536, 65536, 65536, 65536, 65536, 65536, 65536, 65536];
    let items = [
        ListItem::new(format!("{:<15}:{:>20}", "Output Power", values[0])),
        ListItem::new(format!("{:<15}:{:>20}", "Reflected Power", values[1])),
        ListItem::new(format!("{:<15}:{:>20}", "Drive Level", values[2])),
        ListItem::new(format!("{:<15}:{:>20}", "Temperature", values[3])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 1 Current", values[4])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 2 Current", values[5])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 3 Current", values[6])),
        ListItem::new(format!("{:<15}:{:>20}", "Gan 4 Current", values[7])),
        ListItem::new("."),
        ListItem::new(format!("{:^38}", "[CLEAR]")),
    ];
    let block = List::new(items)
        .block(Block::default().title("DAC").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn offsets<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let values = [65536, 65536, 65536, 65536, 65536, 65536, 65536, 65536];
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
    let block = List::new(items)
        .block(Block::default().title("Offsets").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn control<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let values = [false, false, false, false, false];
    let items = [
        ListItem::new(format!("{:^44}", "[Store to Non Volatile Memory]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Load from Non Volatile Memory]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Alarms Reset]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[SSPA Reset]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[SSPA Disable]")),
        ListItem::new("."),
        ListItem::new(format!(
            "{:<38}:{:>6}",
            "SW Reflected Power protection disable", values[0]
        )),
        ListItem::new(format!(
            "{:<38}:{:>6}",
            "SW Over drive protection disable", values[1]
        )),
        ListItem::new(format!(
            "{:<38}:{:>6}",
            "SW Duty cycle protection disable", values[2]
        )),
        ListItem::new(format!(
            "{:<38}:{:>6}",
            "SW Over temperature protection disable", values[3]
        )),
        ListItem::new(format!(
            "{:<38}:{:>6}",
            "SW Over current protection disable", values[4]
        )),
    ];
    let block = List::new(items)
        .block(Block::default().title("Control").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn compile<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let items = [
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Build]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Clean Build]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Clean Build and Flash]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Build and Flash]")),
        ListItem::new("."),
        ListItem::new(format!("{:^44}", "[Flash]")),
    ];
    let block = List::new(items)
        .block(Block::default().title("Compile").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(block, chunk);
}

fn terminal<B: Backend>(chunk: Rect, f: &mut Frame<B>) {
    let text = Text::from("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n23\n24\n25\n26\n27\n28\n29\n30\n31\n32\n33\n34\n35\n36\n37\n38\n39\n40\n41\n42\n43\n44");
    let block = Paragraph::new(text)
        .block(Block::default().title("Terminal").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });
    f.render_widget(block, chunk);
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = chunks(f);
    status(chunks[0], f);
    adc_measurements(chunks[1], f);
    registers(chunks[2], f);
    state(chunks[3], f);
    firmware_version(chunks[5], f);
    hard_reset(chunks[6], f);
    ext_signals(chunks[7], f);
    ext_signals_presets(chunks[8], f);
    dac(chunks[9], f);
    offsets(chunks[10], f);
    control(chunks[11], f);
    compile(chunks[12], f);
    terminal(chunks[13], f);
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        ui(f);
    })?;

    thread::sleep(Duration::from_millis(9000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
