mod tx;

pub use tx::*;

pub fn send_serial_dmx(transmitter: &mut dyn SerialDMXTransmitter, data: &[u8]) {
    transmitter.send_break();
    transmitter.write(data);
}