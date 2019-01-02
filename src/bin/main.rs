extern crate sc_client;
extern crate rosc;
extern crate config;

use std::env;
use std::net::{UdpSocket, SocketAddrV4};
use std::str::FromStr;
use rosc::OscPacket;
use sc_client::server;
use sc_client::server::{Server, options::Options};
use config::{Config, File};

fn main() {
    let config = init_config();
    let options = Options::new(&config);
    let server = Server::new(options);
    server.say_hello();

    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage {} IP:PORT", &args[0]);
    if args.len() < 2 {
        println!("{}", usage);
        ::std::process::exit(1)
    }
    let addr = match SocketAddrV4::from_str(&args[1]) {
        Ok(addr) => addr,
        Err(_) => panic!(usage),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn init_config() -> Config {
    let mut config = Config::new();
    let config_file = File::with_name("Config");
    config
        .merge(config_file)
        .expect("Error reading Config.toml")
        .to_owned()
}


fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            match msg.args {
                Some(args) => {
                    println!("OSC arguments: {:?}", args);
                }
                None => println!("No arguments in message."),
            }
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}
