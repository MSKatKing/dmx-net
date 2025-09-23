mod tx;

pub use tx::*;

pub fn send_serial_dmx(transmitter: &mut dyn SerialDMXTransmitter, data: &[u8]) {
    let mut dmx_frame = [0u8; 513];
    let data_len = data.len().min(512);

    dmx_frame[1..=data_len].copy_from_slice(&data[..data_len]);

    transmitter.send_break();
    transmitter.write(&dmx_frame);
}