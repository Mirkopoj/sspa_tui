use crate::sspa::{Register, RegisterState, SSPAState};
use tui::style::Color;

pub trait ColorTrait {
    fn color(&self) -> Color;
}

impl ColorTrait for Register {
    fn color(&self) -> Color {
        match self.state() {
            RegisterState::Ok => Color::White,
            RegisterState::Warning => Color::Yellow,
            RegisterState::ParityError => Color::Red,
        }
    }
}

impl ColorTrait for bool {
    fn color(&self) -> Color {
        if *self {
            Color::White
        } else {
            Color::DarkGray
        }
    }
}

impl ColorTrait for SSPAState {
    fn color(&self) -> Color {
        match *self {
            SSPAState::Invalid | SSPAState::Failure | SSPAState::Protection => Color::Red,
            SSPAState::Warning | SSPAState::ProtectionHW => Color::Yellow,
            SSPAState::Boot => Color::Blue,
            SSPAState::StandBy | SSPAState::Nominal | SSPAState::Disabled => Color::White,
        }
    }
}
