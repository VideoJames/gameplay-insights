use std::error::Error;
use insights_sdk::{ Envelope, Event, FieldValue, Payload};

fn main() -> Result<(), Box<dyn Error>> {
    // Build an event

    let event = Event::new("First Event")
        .field("Reaction Text", FieldValue::Text("React!".to_string()))
        .field("Firing", FieldValue::Bool(true))
        .field("Second", FieldValue::Bool(true))
        .field("Damage", FieldValue::Int(1))
        .field("X Position", FieldValue::Float(1.5));

    // Wrap it in an Envelope
    let envelope = Envelope::new(0, Payload::Event(event));

    // Encode
    let encoded = envelope.encode()?;

    // Print
    println!("{:?}", encoded);

    Ok(())
}
