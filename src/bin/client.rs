use std::env::{self, Args};
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    const SOCKET_PATH: &str = "/tmp/pen_utils";

    let arg = env::args();
    
    let message = match verify_args(arg) {
        Some(v) => v,
        None => {
            return Ok(())
        }
    };
    
    let mut stream = UnixStream::connect(SOCKET_PATH)?;   

    stream.write_all(message.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)?;

    println!("Response: {response}");
    Ok(())
}

fn verify_args(arg: Args) -> Option<String> {
    if arg.len() < 2 {
        help();
        return None;
    }
    let vect = fmt_args(arg);

    if !vect[0].contains("change") && !vect[0].contains("mode") {
       help();
       return None
    }
    if vect[0].contains("mode") && vect.len() < 2 {
       println!("The command mode have 1 argument");
       return None
    }
    if !vect[1].contains("track") && !vect[1].contains("manual") {
       println!("The argument it's incorrect");
       return None
    }
    Some(vect.join(" "))
}

fn fmt_args(arg: Args) -> Vec<String> {
    let raw: Vec<String> = arg.collect();
    let raw = raw.as_slice().split_first().take().unwrap().1;
    raw.to_vec()
}

fn help() {
    println!(
    "
    \t\t--- Help ---
    Arguments:
        mode -> Change the mode
        change -> Change the select monitor only in the manual mode
        help -> To show this message
    Modes:
        track -> Follow the focused monitor(change with the other medium is not the pen)
        manual -> Change the monitor called(can be manual name or the settets in the server)
    Examples:
        mode track
        ------
        change
    "
    )
}


