mod tx;

use serialport::SerialPort;
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

#[cfg(feature = "std")]
pub fn open_port(port_name: impl Into<String>) -> DMXSerialPort<dyn SerialPort> {
    let port = serialport::new(port_name.into().as_str(), 250000)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::Two)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .open().expect("Failed to open serial port");

    DMXSerialPort(port)
}