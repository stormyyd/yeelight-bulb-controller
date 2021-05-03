use std::env;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};
use std::time;

use rand::Rng;

use command::Command;

mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Usage: {} [IP_ADDRESS:PORT] [METHOD] [PARAM1] [PARAM2] ...",
            args[0]
        );
        return;
    }

    let address = &args[1].parse::<SocketAddr>().expect("Invalid IP address");
    let c = Command {
        id: rand::thread_rng().gen::<u16>(),
        method: args[2].clone(),
        params: args[3..]
            .iter()
            .map(|p| match p.parse::<command::Integer>() {
                Ok(i) => command::Params::Integer(i),
                Err(_) => command::Params::String(p.clone()),
            })
            .collect(),
    };

    match send_command(&address, &c) {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("{}", e),
    };
}

fn send_command(address: &SocketAddr, c: &Command) -> io::Result<String> {
    let message = c.to_json();
    let stream = TcpStream::connect_timeout(&address, time::Duration::from_secs(5))?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    // About why I send command message and "\r\n" separately
    //
    // If you send command message with "\r\n" together after the TCP
    // connection established immediately, your bulb will neither execute
    // your command nor send response back.
    //
    // I don't know why but I think maybe it is a bug from Yeelight.
    //
    // And then, I found two solutions:
    // 1. Send command message and "\r\n" separately as I did.
    // 2. Delay at least 0.1 second after the TCP connection established.
    writer.write_all(message.as_bytes())?;
    writer.flush()?;
    writer.write_all(b"\r\n")?;
    writer.flush()?;

    let mut received_message = String::new();
    reader.read_line(&mut received_message)?;
    Ok(received_message)
}
