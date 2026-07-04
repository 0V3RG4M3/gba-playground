use gba::gba_cell::GbaCellSafe;
use gba::keys::KeyInput;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct RxState(u32);

impl RxState {
    pub const fn new() -> Self {
        RxState(0)
    }

    pub fn connected(self) -> bool {
        self.0 >> 0 & 1 != 0
    }

    pub fn with_connected(mut self, connected: bool) -> Self {
        self.0 &= !(1 << 0);
        self.0 |= u32::from(connected) << 0;
        self
    }

    /*pub fn parent(self) -> bool {
        self.0 >> 1 & 1 != 0
    }*/

    pub fn with_parent(mut self, parent: bool) -> Self {
        self.0 &= !(1 << 1);
        self.0 |= u32::from(parent) << 1;
        self
    }

    /*pub fn frame(self) -> u8 {
        (self.0 >> 2 & 0xf) as u8
    }*/

    pub fn with_frame(mut self, frame: u8) -> Self {
        self.0 &= !(0xf << 2);
        self.0 |= u32::from(frame) << 2;
        self
    }

    /*pub fn key_inputs(self) -> [KeyInput; 2] {
        let ki0 = (self.0 >> 6 & 0x3ff) as u16;
        let ki1 = (self.0 >> 16 & 0x3ff) as u16;
        [ki0.into(), ki1.into()]
    }*/

    pub fn with_key_inputs(mut self, key_inputs: [KeyInput; 2]) -> Self {
        self.0 &= !(0x3ff << 6);
        self.0 &= !(0x3ff << 16);
        self.0 |= u32::from(u16::from(key_inputs[0])) << 6;
        self.0 |= u32::from(u16::from(key_inputs[1])) << 16;
        self
    }
}

unsafe impl GbaCellSafe for RxState {}
