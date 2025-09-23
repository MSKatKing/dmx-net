/// This trait should be implemented on any type that can interface with a raw physical DMX port.
pub trait SerialDMXTransmitter {
    /// This method should send a LOW value for at least 88us followed by a HIGH value for at least 8us
    fn send_break(&mut self);

    /// This method should write the bytes of data to the DMX port one at a time.
    fn write(&mut self, data: &[u8]);
}

#[cfg(feature = "std")]
impl<Port: serialport::SerialPort> SerialDMXTransmitter for Port {
    fn send_break(&mut self) {
        self.write_data_terminal_ready(false).expect("Failed to write low to serialport::SerialPort");
        std::thread::sleep(std::time::Duration::from_micros(100));
        self.write_data_terminal_ready(true).expect("Failed to write high to serialport::SerialPort");
        std::thread::sleep(std::time::Duration::from_micros(12));
    }

    fn write(&mut self, data: &[u8]) {
        self.write(data).expect("Failed to write data to serialport::SerialPort");
    }
}