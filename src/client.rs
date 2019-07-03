use std::thread;
use std::io::{self, prelude::*};
use std::net::{TcpStream, SocketAddrV4};
use std::char;

pub fn run(name: String, address: SocketAddrV4) -> io::Result<()> {
    let mut write_stream = TcpStream::connect(address)?;
    let read_stream = write_stream.try_clone()?;
    let name_copy = name.clone();
    thread::spawn(|| {
        read_from_server(read_stream, name_copy);
    });

    // listen to stdin for new messages
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = format!("{}: {}", name, line);
        write_stream.write(line.as_bytes())?;
    }

    unreachable!("stdin closed!?")
}

fn read_from_server(mut read_stream: TcpStream, name: String) {
    loop {
        let mut buffer = [0;128];
        if let Ok(_) = read_stream.read(&mut buffer) {
            print_message(buffer, name.as_str());
        } else {
            println!("lost tcp connection to server!");
            break;
        }
    }
}

// takes in a u8 buffer and checks to see if it is empty by looking
// at the first char (there is probably a better way to do this) and
// prints the string if it is not empty.
fn print_message(buffer: [u8;128], name: &str) {
    let message = format!("{}",String::from_utf8_lossy(&buffer[..]));
    if message.chars().next().unwrap() != char::from_u32(0).unwrap()
        && !message.as_str().starts_with(name)
    {
        println!("{}", message);
    }
}
