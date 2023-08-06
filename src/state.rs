use tui::widgets::ListState;

use crate::{
    sspa::{Register, SSPAState},
    ui::WidgetId,
};

pub struct StateKeeper {
    status_register: Register,
    adc: [Register; 8],
    thresholds: [Register; 10],
    sspa_state: SSPAState,
    version_number: Register,
    powen: bool,
    current_tnr: [u16; 3],
    cache_tnr: [u16; 3],
    dac: [u16; 8],
    offsets: [u16; 8],
    control_register: Register,
    selected_widget: WidgetId,
    widget_transition: [[WidgetId; 4]; 8],
    list_state: [ListState; 8],
    list_element_count: [usize; 8],
}

pub enum StateTransition {
    Left,
    Down,
    Up,
    Right,
}

pub type ExtSignals = (bool, [u16; 3], [u16; 3]);

impl StateKeeper {
    pub fn new() -> StateKeeper {
        let mut list_state_1 = ListState::default();
        list_state_1.select(Some(0));
        StateKeeper {
            status_register: Register::new(0),
            adc: [Register::new(0); 8],
            thresholds: [Register::new(0); 10],
            sspa_state: SSPAState::Invalid,
            version_number: Register::new(0),
            powen: true,
            current_tnr: [0; 3],
            cache_tnr: [0; 3],
            dac: [0; 8],
            offsets: [0; 8],
            control_register: Register::new(0),
            selected_widget: WidgetId::Ext,
            widget_transition: [
                [
                    WidgetId::Registers,
                    WidgetId::HardReset,
                    WidgetId::Registers,
                    WidgetId::ExtPresets,
                ],
                [
                    WidgetId::HardReset,
                    WidgetId::HardReset,
                    WidgetId::Registers,
                    WidgetId::Compile,
                ],
                [
                    WidgetId::Registers,
                    WidgetId::ExtPresets,
                    WidgetId::Ext,
                    WidgetId::Dac,
                ],
                [
                    WidgetId::Registers,
                    WidgetId::Compile,
                    WidgetId::Ext,
                    WidgetId::Offsets,
                ],
                [
                    WidgetId::HardReset,
                    WidgetId::Compile,
                    WidgetId::ExtPresets,
                    WidgetId::Offsets,
                ],
                [
                    WidgetId::Ext,
                    WidgetId::Offsets,
                    WidgetId::Dac,
                    WidgetId::Control,
                ],
                [
                    WidgetId::ExtPresets,
                    WidgetId::Offsets,
                    WidgetId::Dac,
                    WidgetId::Control,
                ],
                [
                    WidgetId::Dac,
                    WidgetId::Control,
                    WidgetId::Control,
                    WidgetId::Control,
                ],
            ],
            list_state: [
                ListState::default(),
                ListState::default(),
                list_state_1,
                ListState::default(),
                ListState::default(),
                ListState::default(),
                ListState::default(),
                ListState::default(),
            ],
            list_element_count: [
                10,
                1,
                8,
                1,
                5,
                9,
                8,
                10,
            ],
        }
    }

    pub fn status_register(&self) -> Register {
        self.status_register
    }

    pub fn adc_measurements(&self) -> [Register; 8] {
        self.adc
    }

    pub fn thresholds(&self) -> [Register; 10] {
        self.thresholds
    }

    pub fn sspa_state(&self) -> SSPAState {
        self.sspa_state
    }

    pub fn version_number(&self) -> Register {
        self.version_number
    }

    pub fn ext_signals(&self) -> ExtSignals {
        (self.powen, self.current_tnr, self.cache_tnr)
    }

    pub fn dac(&self) -> [u16; 8] {
        self.dac
    }

    pub fn offsets(&self) -> [u16; 8] {
        self.offsets
    }

    pub fn control_register(&self) -> Register {
        self.control_register
    }

    pub fn widget_select(&mut self, transition: Option<StateTransition>) {
        if let Some(state_transition) = transition {
            self.selected_widget =
                self.widget_transition[self.selected_widget as usize][state_transition as usize];
            self.list_state[self.selected_widget as usize].select(Some(0));
        }
    }

    pub fn is_widget_selected(&self, wid: WidgetId) -> bool {
        wid == self.selected_widget
    }

    pub fn selected_item(&mut self, wid: WidgetId) -> &mut ListState {
        if !self.is_widget_selected(wid) {
            self.list_state[wid as usize].select(None);
        }
        &mut self.list_state[wid as usize]
    }

    pub fn element_select(&mut self, transition: Option<StateTransition>) {
        if let Some(state_transition) = transition {
            match state_transition {
                StateTransition::Up => {
                    for e in &mut self.list_state {
                        if let Some(n) = e.selected() {
                            let n = n.saturating_sub(1);
                            e.select(Some(n));
                        }
                    }
                },
                StateTransition::Down => {
                    for e in &mut self.list_state {
                        if let Some(n) = e.selected() {
                            let n = (n+1).clamp(0, self.list_element_count[self.selected_widget as usize] - 1);
                            e.select(Some(n));
                        }
                    }
                },
                _ => { },
            }
        }
    }

}
