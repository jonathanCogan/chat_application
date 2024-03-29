use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::sync::mpsc;
use std::io::prelude::*;

use crate::error::Error;

pub enum BcMsg {
    NewUser(mpsc::Sender<String>),
    Broadcast(String),
}

pub fn run(address: SocketAddrV4) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;
    let (bc_sender, bc_receiver) = mpsc::channel();
    thread::spawn(|| {
        handle_broadcast(bc_receiver);
    });

    for stream in listener.incoming() {
        let write_stream = stream?;
        let read_stream = write_stream.try_clone()?;
        let bc_sender_clone = bc_sender.clone();
        let (th_sender, th_receiver) = mpsc::channel();
        thread::spawn(|| {
            read_from_client(read_stream, bc_sender_clone);
        });

        thread::spawn(|| {
            write_to_client(write_stream, th_receiver);
        });

        bc_sender.send(BcMsg::NewUser(th_sender))?;
    }

    unreachable!("ALWAYS LISTENING")
}

fn read_from_client(mut read_stream: TcpStream, bc_sender: mpsc::Sender<BcMsg>) {
    loop {
        // TODO: Use `BufReader` and `read_line`.
        // TODO: Notify other side of closed connection for graceful shutdown.
        let mut buffer = [0; 128];
        if let Ok(_) = read_stream.read(&mut buffer) {
            let message = String::from_utf8_lossy(&buffer).to_string();
            bc_sender.send(BcMsg::Broadcast(message)).expect("bc_sender");
        } else {
            println!("lost tcp connection to client");
            break;
        }
    }
}

fn write_to_client(mut write_stream: TcpStream, bc_receiver: mpsc::Receiver<String>) {
    loop {
        if let Ok(message) = bc_receiver.recv() {
            if write_stream.write(message.as_bytes()).is_err() {
                println!("lost connection to bc thread");
                break;
            }
        } else {
            println!("lost connection to bc thread");
            break;
        }
    }
}

fn handle_broadcast(receiver: mpsc::Receiver<BcMsg>) {
    let mut users: Vec<mpsc::Sender<String>> = Vec::new();
    loop {
        match receiver.recv() {
            Ok(BcMsg::NewUser(user)) => users.push(user),
            Ok(BcMsg::Broadcast(msg)) => {
                users.retain(|user| {
                    match user.send(msg.clone()) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                });
            },
            Err(e) => eprintln!("Error: {}", e)
        }
    }
}
