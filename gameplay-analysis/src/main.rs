use insights_sdk::transport::{read_frame, StreamError};
use std::error::Error;
use std::io::ErrorKind;
use std::net::TcpListener;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;
    let (mut stream, address) = listener.accept()?;
    println!("client connected from {}", address);

    loop {
        match read_frame(&mut stream) {
            Ok(envelope) => println!("{:?}", envelope),
            Err(StreamError::IO(e)) if e.kind() == ErrorKind::UnexpectedEof => {
                println!("client disconnected");
                break;
            }
            Err(other) => return Err(other.into())
        }
    }

    Ok(())
}
