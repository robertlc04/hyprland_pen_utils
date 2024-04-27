use std::os::unix::net::{UnixListener, UnixStream};
use std::io::prelude::*;
use std::process::Command;
// use std::{env, thread};

// use hyprland::keyword::{Keyword, OptionValue};
use hyprland::Result;

use crate::structs::{Modes, SMonitors};

mod structs;

fn main() -> std::io::Result<()> {
    const SOCKET_PATH: &str = "/tmp/pen_utils";
    let mut count_messages = 0;

    let mut actual_monitor = SMonitors::new();
    let mut actual_mode = Modes::new(&mut actual_monitor);

    // Set input:tablet Keyword
    // let _ = Keyword::set("input:tablet:output", "eDP-1");

    
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
        let _ = change(modes);
    }

    stream.write(b"recived")?;

    Ok(())
}

// TODO: Try to make other alternative to don't exec
fn change(modes: &mut Modes) -> Result<()>  {

    let mut hyprctl = Command::new("hyprctl");

    const HDMI: &str = "keyword input:tablet:output HDMI-A-1";
    const MAIN: &str = "keyword input:tablet:output eDP-1";


    if modes.eq_mon("main".to_string()) {
            hyprctl
            .arg(HDMI)
            .spawn()?;
        modes.switch_mon();
        return Ok(())
        // println!("{modes:?}")
    }

    if modes.eq_mon("hdmi".to_string()) {
        hyprctl
            .arg(MAIN)
            .spawn()?;
        modes.switch_mon();
        return Ok(())
        // println!("{modes:?}")
    }

    // println!("ENTRE");
    // let border_size = match Keyword::get("input:tablet:output") {
    // let border_size = match Keyword::get("general:gaps_in") {
    //     Ok(x) => println!("{x:?}"),
    //     Err(e) => println!("{e:?}"),
    // };
    // let keyw = Keyword::get("input:tablet:output").unwrap().value;
    // println!("VALUEEE {border_size}");
    Ok(())
}

fn mode(mode: String, modes: &mut Modes) {
    // if !modes.eq(mode) {
    //     modes.switch()
    // }
    todo!()
}



