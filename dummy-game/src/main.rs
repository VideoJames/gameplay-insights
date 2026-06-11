use std::error::Error;
use std::net::TcpStream;
use insights_sdk::{Envelope, Event, FieldValue, Payload};
use insights_sdk::transport::write_frame;

fn main() -> Result<(), Box<dyn Error>> {
    // Build an event

    let event_a = Event::new("First Event")
        .field("Reaction Text", FieldValue::Text("React!".to_string()))
        .field("Firing", FieldValue::Bool(true))
        .field("Second", FieldValue::Bool(true))
        .field("Damage", FieldValue::Int(1))
        .field("X Position", FieldValue::Float(1.5));

    let event_b = Event::new("B Event");

    // Wrap it in an Envelope
    let envelope_a = Envelope::new(0, Payload::Event(event_a));
    let envelope_b = Envelope::new(0, Payload::Event(event_b));

    let mut stream = TcpStream::connect("127.0.0.1:8088")?;

    write_frame(&mut stream, &envelope_a)?;
    write_frame(&mut stream, &envelope_b)?;

    Ok(())
}
