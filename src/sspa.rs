#[derive(Clone, Copy)]
pub enum RegisterState {
    ParityError,
    Warning,
    Ok,
}

#[derive(Clone, Copy)]
pub struct Register {
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
        let value = value&0x7FFF;
        Register { state, value }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn state(&self) -> RegisterState {
        self.state
    }

}

pub fn bits(reg: &Register) -> [bool; 15] {
    (0..15)
        .rev()
        .map(|n| (reg.value & (1 << n)) != 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SSPAState {
    Invalid,
    Boot,
    StandBy,
    Failure,
    Disabled,
    Nominal,
    Warning,
    ProtectionHW,
    Protection,
}

