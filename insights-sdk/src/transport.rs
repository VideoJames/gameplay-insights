use std::io::{Read, Write};
use crate::Envelope;

#[derive(Debug)]
pub enum StreamError {
    SerdeJSON(serde_json::Error),
    IO(std::io::Error),
}

impl From<serde_json::Error> for StreamError {
    fn from(err: serde_json::Error) -> Self {
        StreamError::SerdeJSON(err)
    }
}

impl From<std::io::Error> for StreamError {
    fn from(err: std::io::Error) -> Self {
        StreamError::IO(err)
    }
}
pub fn write_frame(write: &mut impl Write, envelope: &Envelope) -> Result<(), StreamError> {
    let bytes = envelope.encode()?;

    let length = bytes.len() as u32;

    let mut frame = Vec::new();
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(bytes.as_slice());

    write.write_all(frame.as_slice())?;

    Ok(())
}

pub fn read_frame(reader: &mut impl Read) -> Result<Envelope, StreamError> {
    let mut len_buf = [0u8; 4]; // Read the first four bytes for the payload length
    reader.read_exact(&mut len_buf)?;

    let length = u32::from_be_bytes(len_buf);

    let mut payload_buf = vec![0u8; length as usize];
    reader.read_exact(&mut payload_buf)?;

    let envelope = Envelope::decode(payload_buf)?;

    Ok(envelope)
}