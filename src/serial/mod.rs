mod tx;

pub use tx::*;

/// Sends DMX data using a raw serial transmitter.
///
/// `data` should contain up to 512 channel values. If fewer than 512 bytes
/// are provided, the rest are filled with zeros.
pub fn send_serial_dmx(transmitter: &mut dyn SerialDMXTransmitter, data: &[u8]) {
    let mut dmx_frame = [0u8; 513];
    let data_len = data.len().min(512);

    dmx_frame[1..=data_len].copy_from_slice(&data[..data_len]);

    transmitter.send_break();
    transmitter.write(&dmx_frame);
}