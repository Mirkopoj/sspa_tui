use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};

use crate::state::{StateKeeper, StateTransition};

pub struct Events {
    rx: Receiver<Event>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = channel(128);
        tokio::spawn(async move {
            event_thread(tx)
                .await
                .expect("ERROR: crossterm event reader failed");
        });
        Events { rx }
    }

    pub fn hanlde(&mut self, state: &mut StateKeeper) -> bool {
        match self.rx.try_recv() {
            Ok(events) => {
                state.widget_select(widget_selection(&events));
                state.element_select(element_selection(&events));
                false
            }
            Err(e) => match e {
                TryRecvError::Empty => false,
                _ => true,
            },
        }
    }
}

fn widget_selection(event: &Event) -> Option<StateTransition> {
    if let Event::Key(key) = event {
        if key.modifiers != KeyModifiers::CONTROL {
            return None;
        }
        match key.code {
            KeyCode::Up => return Some(StateTransition::Up),
            KeyCode::Down => return Some(StateTransition::Down),
            KeyCode::Left => return Some(StateTransition::Left),
            KeyCode::Right => return Some(StateTransition::Right),
            KeyCode::Char(c) => match c {
                'k' | 'K' => return Some(StateTransition::Up),
                'j' | 'J' => return Some(StateTransition::Down),
                'h' | 'H' => return Some(StateTransition::Left),
                'l' | 'L' => return Some(StateTransition::Right),
                _ => {}
            },
            _ => {}
        }
    }
    None
}

fn element_selection(event: &Event) -> Option<StateTransition> {
    if let Event::Key(key) = event {
        if key.modifiers != KeyModifiers::NONE {
            return None;
        }
        match key.code {
            KeyCode::Up => return Some(StateTransition::Up),
            KeyCode::Down => return Some(StateTransition::Down),
            KeyCode::Left => return Some(StateTransition::Left),
            KeyCode::Right => return Some(StateTransition::Right),
            KeyCode::Char(c) => match c {
                'k' | 'K' => return Some(StateTransition::Up),
                'j' | 'J' => return Some(StateTransition::Down),
                'h' | 'H' => return Some(StateTransition::Left),
                'l' | 'L' => return Some(StateTransition::Right),
                _ => {}
            },
            _ => {}
        }
    }
    None
}

async fn event_thread(tx: Sender<Event>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let event = read()?;
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Char(c) => match c {
                        'q' | 'Q' => {
                            break;
                        }
                        _ => {}
                    },
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
                tx.send(event).await?;
            }
            Event::Mouse(_) => {
                tx.send(event).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
