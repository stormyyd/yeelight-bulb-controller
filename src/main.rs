use rand::Rng;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::time;

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

    let address = {
        let address = &args[1];
        address.parse::<SocketAddr>().expect("Invalid IP address")
    };
    let mut c = {
        let method = &args[2];
        let id = rand::thread_rng().gen::<u16>();
        command::Command::new(id, method)
    };
    for param in args[3..].iter() {
        match param.parse::<command::Integer>() {
            Ok(result) => c.params.push(command::Params::Integer(result)),
            Err(_) => c.params.push(command::Params::String(String::from(param))),
        }
    }
    let message = c.to_json().expect("encode command message error");

    let mut stream =
        TcpStream::connect_timeout(&address, time::Duration::new(5, 0)).expect("connection failed");
    stream
        .write_all(message.as_bytes())
        .expect("cannot send message to your bulb");
    stream
        .write_all(b"\r\n")
        .expect("cannot send message to your bulb");
    let mut reader = BufReader::new(stream);
    let mut received_message = String::new();
    reader
        .read_line(&mut received_message)
        .expect("cannot read message from your bulb");
    println!("{}", received_message);
}
