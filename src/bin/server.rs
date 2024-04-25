use std::os::unix::net::{UnixListener, UnixStream};
use std::io::prelude::*;

use crate::structs::Modes;

mod structs;

fn main() -> std::io::Result<()> {
    const SOCKET_PATH: &str = "/tmp/pen_utils";
    let mut count_messages = 0;

    let mut actual_mode = Modes::new();

    // Verify if the socket exists
    if std::fs::metadata(SOCKET_PATH).is_ok() {
        println!("Deleting socket");
        std::fs::remove_file(SOCKET_PATH)?;
    }

    // Create the socket
    let unix_listener = UnixListener::bind(SOCKET_PATH)?;
    
    // Main Loop
    for stream in unix_listener.incoming() {
        match stream {
            Ok(stream) => {
                handler(stream, &mut count_messages,&mut actual_mode)?;
            },
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
    }

    Ok(())
}

fn handler(mut stream: UnixStream,count_messages: &mut i32, modes: &mut Modes) -> std::io::Result<()> {
    let mut message = String::new();

    stream.read_to_string(&mut message)?;
    *count_messages += 1;
    println!("{count_messages} - {message}");

    let smessage: Vec<&str> = message.split_whitespace().collect();

    if smessage[0].contains("mode") {
        mode(smessage[1].to_string(), modes)
    }
    if smessage[0].contains("change") {
        change()
    }

    stream.write(b"recived")?;

    Ok(())
}

fn change() {
    todo!()
}

fn mode(mode: String, modes: &mut Modes) {
    if !modes.eq(mode) {

    }
    todo!()
}



