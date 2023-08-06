mod color;
mod events;
mod sspa;
mod ui;
mod state;
mod launcher;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::Events;
use state::StateKeeper;

use std::io;

use tui::{backend::CrosstermBackend, Terminal};

use ui::ui;

use launcher::Launcher;


#[tokio::main]
async fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();
    let mut state = StateKeeper::new();
    let mut term = Launcher::new(47);
    term.launch("ping localhost");
    let mut ssh = Launcher::new(20);
    ssh.launch("ssh -tt dietpi@192.168.1.16 sspa -v -H -M");
    //ssh.launch("ssh -tt dietpi@192.168.1.16 ping 192.168.1.15");

    loop {
        if events.hanlde(&mut state) {
            break;
        }
        terminal.draw(|f| {
            ui(f, &mut state, &mut term, &mut ssh);
        })?;
    }

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
