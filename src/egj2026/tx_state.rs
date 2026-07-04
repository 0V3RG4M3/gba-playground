use gba::gba_cell::GbaCellSafe;
use gba::keys::KeyInput;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct TxState(u16);

impl TxState {
    pub const fn new() -> Self {
        TxState(0)
    }

    pub fn frame(self) -> u8 {
        (self.0 >> 0 & 0xf) as u8
    }

    pub fn with_frame(mut self, frame: u8) -> Self {
        self.0 &= !(0xf << 0);
        self.0 |= u16::from(frame) << 0;
        self
    }

    pub fn key_input(self) -> KeyInput {
        let ki = self.0 >> 4 & 0x3ff;
        ki.into()
    }

    pub fn with_key_inputs(mut self, key_input: KeyInput) -> Self {
        self.0 &= !(0x3ff << 4);
        self.0 |= u16::from(key_input) << 4;
        self
    }
}

impl From<u16> for TxState {
    fn from(state: u16) -> TxState {
        TxState(state)
    }
}

impl From<TxState> for u16 {
    fn from(state: TxState) -> u16 {
        state.0
    }
}

unsafe impl GbaCellSafe for TxState {}
