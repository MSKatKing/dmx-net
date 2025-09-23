/// This trait should be implemented on any type that can interface with a raw physical DMX port.
///
/// # DMX512 Serial Port Specifications
///
/// A compliant DMX transmitter must follow these physical and protocol-level settings:
///
/// - **Baud Rate**: 250,000 bps
/// - **Data Bits**: 8
/// - **Parity**: None
/// - **Stop Bits**: 2
/// - **Flow Control**: None
/// - **Electrical Standard**: RS-485 (half-duplex)
///
/// # DMX Frame Structure
/// Each DMX frame must be preceded by a "break" and "mark-after-break" (MAB):
///
/// 1. **Break**: The line must be held LOW for **at least 88 µs**
/// 2. **Mark After Break (MAB)**: The line must be HIGH for **at least 8 µs**
/// 3. **Start Code**: The first byte after MAB (usually 0x00, but others exist supporting more features)
/// 4. **Channel Data**: Up to 512 bytes (representing DMX channel values)
///
/// # Notes
///
/// This trait is automatically implemented on `DMXSerialPort` when the std feature is enabled.
/// In no_std environments, it must be implemented by the user.
///
/// To get a DMX512 compliant port using `serialport::SerialPort`, use the `open_port` function.

pub trait SerialDMXTransmitter {
    /// Sends the DMX 'break' and 'mark-after-break' (MAB) signals.
    ///
    /// This method should hold the line LOW for at least 88 microseconds (DMX break),
    /// followed by HIGH for at least 8 microseconds (mark-after-break).
    fn send_break(&mut self);

    /// This method should write the bytes of data to the DMX port one at a time.
    fn write(&mut self, data: &[u8]);
}

#[cfg(feature = "std")]
pub struct DMXSerialPort<Port: serialport::SerialPort + ?Sized>(pub(super) Box<Port>);

#[cfg(feature = "std")]
impl<Port: serialport::SerialPort> SerialDMXTransmitter for DMXSerialPort<Port> {
    fn send_break(&mut self) {
        self.write_data_terminal_ready(false).expect("Failed to write low to serialport::SerialPort");
        std::thread::sleep(std::time::Duration::from_micros(100));
        self.write_data_terminal_ready(true).expect("Failed to write high to serialport::SerialPort");
        std::thread::sleep(std::time::Duration::from_micros(12));
    }

    fn write(&mut self, data: &[u8]) {
        use core::ops::DerefMut;

        self.deref_mut().write(data).expect("Failed to write data to serialport::SerialPort");
    }
}

#[cfg(feature = "std")]
impl<Port: serialport::SerialPort> core::ops::Deref for DMXSerialPort<Port> {
    type Target = Port;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "std")]
impl<Port: serialport::SerialPort> core::ops::DerefMut for DMXSerialPort<Port> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}