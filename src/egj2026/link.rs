use gba::gba_cell::GbaCell;
use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mmio;

use crate::egj2026::rx_state::RxState;
use crate::egj2026::tx_state::TxState;

pub fn init() {
    let mut rcnt = mmio::RCNT.read();
    rcnt &= !(1 << 14);
    rcnt &= !(1 << 15);
    mmio::RCNT.write(rcnt);

    let mut siocnt = mmio::SIOCNT.read();
    siocnt &= !(1 << 12);
    siocnt |= 1 << 13;
    siocnt |= 1 << 14;
    mmio::SIOCNT.write(siocnt);
}

pub fn write(tx_state: TxState) {
    gba::RUST_IRQ_HANDLER.write(Some(irq_handler));

    let mut siocnt = mmio::SIOCNT.read();
    let parent = (siocnt >> 2) & 1 == 0;
    let ready = (siocnt >> 3) & 1 != 0;
    if parent {
        let packet = tx_state.into();
        mmio::SIOMLT_SEND.write(packet);

        if ready {
            siocnt |= 1 << 7;
            mmio::SIOCNT.write(siocnt);
        }
    } else {
        TX_STATE.write(tx_state);
    }
}

pub fn read() -> RxState {
    RX_STATE.read()
}

#[unsafe(link_section = ".iwram")]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if !irq_bits.serial() {
        return;
    }

    let siocnt = mmio::SIOCNT.read();
    let parent = (siocnt >> 2) & 1 == 0;
    let ready = (siocnt >> 3) & 1 != 0;
    let packets = [
        mmio::SIOMULTI0.read(),
        mmio::SIOMULTI1.read(),
        mmio::SIOMULTI2.read(),
        mmio::SIOMULTI3.read(),
    ];
    let frames = [((packets[0] >> 0) & 0xf) as u8, ((packets[1] >> 0) & 0xf) as u8];
    let mut connected = true;
    connected &= ready;
    connected &= packets[0] != u16::MAX;
    connected &= packets[1] != u16::MAX;
    connected &= packets[2] == u16::MAX;
    connected &= packets[3] == u16::MAX;
    connected &= frames[0] == frames[1];
    let key_inputs = [KeyInput::from(packets[0] >> 4), KeyInput::from(packets[1] >> 4)];
    let rx_state = RxState::new()
        .with_connected(connected)
        .with_parent(parent)
        .with_frame(frames[0])
        .with_key_inputs(key_inputs);
    RX_STATE.write(rx_state);

    if !parent {
        let tx_state = TX_STATE.read();
        let packet = tx_state.into();
        mmio::SIOMLT_SEND.write(packet);
    }
}

static TX_STATE: GbaCell<TxState> = GbaCell::new(TxState::new());
static RX_STATE: GbaCell<RxState> = GbaCell::new(RxState::new());
