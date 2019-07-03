use std::thread;
use std::net::{TcpListener, TcpStream, SocketAddrV4};
use std::sync::mpsc;
use std::io::prelude::*;

enum BcMsg {
    NewUser(mpsc::Sender<String>),
    Broadcast(String),
}

pub fn run(address: SocketAddrV4) {
    let listener = TcpListener::bind(address).unwrap();
    let (bc_sender, bc_receiver) = mpsc::channel();
    thread::spawn(|| {
        handle_broadcast(bc_receiver);
    });

    for stream in listener.incoming() {
        let write_stream = stream.unwrap();
        let read_stream = write_stream.try_clone().unwrap();
        let bc_sender_clone = bc_sender.clone();
        let (th_sender, th_receiver) = mpsc::channel();
        thread::spawn(|| {
            read_from_client(read_stream, bc_sender_clone);
        });
        thread::spawn(|| {
            write_to_client(write_stream, th_receiver);
        });
        bc_sender.send(BcMsg::NewUser(th_sender)).unwrap();
    }
}

fn read_from_client(mut read_stream: TcpStream, bc_sender: mpsc::Sender<BcMsg>) {
    loop {
        let mut buffer = [0;128];
        if let Ok(_) = read_stream.read(&mut buffer) {
            let message = String::from_utf8_lossy(&buffer[..]).to_string();
            bc_sender.send(BcMsg::Broadcast(message)).unwrap();
        } else {
            println!("lost tcp connection to client");
            break;
        }
    }
}

fn write_to_client(mut write_stream: TcpStream, bc_receiver: mpsc::Receiver<String>) {
    loop {
        if let Ok(message) = bc_receiver.recv() {
            write_stream.write(message.as_bytes()).unwrap();
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
            Err(_) => panic!("error receiving in bc thread"),
        }
    }
}
