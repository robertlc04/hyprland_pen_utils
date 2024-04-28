use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle, Thread};
use std::{char, env};
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::prelude::*;
use std::process::Command;
// use std::{env, thread};

// use hyprland::keyword::{Keyword, OptionValue};

use crate::structs::{Modes, SMonitors};

mod structs;

fn main() -> std::io::Result<()> {
    const SOCKET_PATH: &str = "/tmp/pen_utils";
    let mut count_messages = 0;

    let mut actual_monitor = SMonitors::new();
    let mut actual_mode = Modes::new(&mut actual_monitor);

    let (tx,rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        track(&rx).expect("fuck");
    });

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
                let smessage = handler(stream, &mut count_messages)?;
                if smessage[0].contains("mode") {
                    mode(smessage[1].to_string(), &mut actual_mode, &tx)?
                }
                if smessage[0].contains("change") {
                    change(&mut actual_mode)?;
                }
            },
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
    }

    Ok(())
}

fn handler(mut stream: UnixStream,count_messages: &mut i32) -> std::io::Result<Vec<String>> {
    let mut message = String::new();

    stream.read_to_string(&mut message)?;
    stream.write(b"Recived")?;
    *count_messages += 1;
    println!("{count_messages} - {message}");

    let smessage: Vec<String> = message.split_whitespace().map(|x| x.to_string()).collect();

    Ok(smessage)
}

// TODO: Try to make other alternative to don't exec
fn change(modes: &mut Modes) -> std::io::Result<()>  {

    let mut hyprctl = Command::new("hyprctl");

    const HDMI: &str = "keyword input:tablet:output HDMI-A-1";
    const MAIN: &str = "keyword input:tablet:output eDP-1";

    if modes.eq_mon("main".to_string()) {
            hyprctl
            .arg(HDMI)
            .spawn()?;
        modes.switch_mon();
        return Ok(())
    }

    if modes.eq_mon("hdmi".to_string()) {
        hyprctl
            .arg(MAIN)
            .spawn()?;
        modes.switch_mon();
        return Ok(())
    }

    Ok(())
}

fn manual_change(cmp: String) -> std::io::Result<()> {
    let mut hyprctl = Command::new("hyprctl");

    const HDMI: &str = "keyword input:tablet:output HDMI-A-1";
    const MAIN: &str = "keyword input:tablet:output eDP-1";

    println!("{cmp}");
    if cmp.contains("eDP-1") {
        println!("LAPTOP");
        hyprctl
            .arg(MAIN)
            .spawn()?;
        return Ok(())
    }
    if cmp.contains("HDMI-A-1") {
        println!("MONITOR");
        hyprctl
            .arg(HDMI)
            .spawn()?;
        return Ok(())
    }

    Ok(())
}

fn mode(mode: String, modes: &mut Modes, sender: &Sender<String> ) -> std::io::Result<()> {
    if !modes.eq_mode(mode) {
        modes.switch_mode()
    }
    sender.send(modes.get_actual()).unwrap();
    if modes.eq_mode("track".to_string()) {
        sender.send("true".to_string()).unwrap();
        // stream.read_to_string(&mut buf)?;
    }
    if modes.eq_mode("manual".to_string()) {
        sender.send("false".to_string()).unwrap();
    }
    Ok(())
}

fn track(reciever: &Receiver<String>) -> std::io::Result<()> {
    let his: String = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or("none".to_string());
    if his.contains("none") {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Not found"))
    }
    let shypr_path = format!("/tmp/hypr/{his}/.socket2.sock");
    let mut follow = false;

    loop {
        match reciever.try_recv() {
            Ok(cmp) => {
                if cmp.eq("true") {
                    follow = true;
                }
                if cmp.eq("false") {
                    follow = false;
                }
                if !cmp.eq("None") && !cmp.eq("true") && !cmp.eq("false") {
                    manual_change(cmp)?;
                }
                
            },
            Err(_) => {}
        }
        if follow {
            let focusedmon = get_focusedmon(shypr_path.clone())?;
            manual_change(focusedmon)?;
        }
        
        if false == true {
            break
        }
    }

   Ok(())

}

fn get_focusedmon(shypr_path: String) -> std::io::Result<String> {
    const BUF_SIZE: usize = 1;

    let mut stream = UnixStream::connect(shypr_path)?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response: Vec<char> = vec![];
    let mut buf = [0;BUF_SIZE];
    let mut focusedmon = String::new();

    loop {
        stream.read_exact(&mut buf)?;
        if buf[0] == 10 {
            let st = String::from_iter(response.clone());
            let st = st.trim();
            if st.contains("focusedmon") {
                focusedmon = st.to_string();
                break;
            }
            response.clear();
        }
        response.append(&mut buf.map(|c| c as char).to_vec());
        if buf.len() == 0 {
            break;
        }
    }

    Ok(focusedmon)
}
