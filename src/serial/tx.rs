/// This trait should be implemented on any type that can interface with a raw physical DMX port.
pub trait SerialDMXTransmitter {
    /// This method should send a LOW value for at least 88us followed by a HIGH value for at least 12us
    fn send_break(&self);

    /// This method should write the bytes of data to the DMX port one at a time.
    fn write(&self, data: &[u8]);
}