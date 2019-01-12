use rosc::{encoder, decoder, OscPacket, OscMessage};
use std::net::{ SocketAddrV4, UdpSocket };
use server::options::Options;
use std::str::FromStr;
use std::thread;
use std::sync::Arc;

pub struct OscHandler {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
}

impl OscHandler {
    pub fn new(options: &Options) -> Self {
        let client_address = SocketAddrV4::from_str(&format!("{}:{}", options.client_address, options.client_port)).unwrap();
        let server_address = SocketAddrV4::from_str(&format!("{}:{}", options.bind_to_address, options.udp_port_number)).unwrap();
        let socket = UdpSocket::bind(client_address).unwrap();
        let osc_handler = OscHandler {
            client_address: client_address,
            server_address: server_address,
            udp_socket: Arc::new(socket),
        };
        osc_handler.start_listener();

        osc_handler
    }

    fn start_listener(&self) {
        let socket = self.udp_socket.clone();
        thread::spawn(move || {
            let mut buf = [0u8; rosc::decoder::MTU];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((size, addr)) => {
                        //TODO check if the address is the server
                        //encapsulate into method
                        println!("Received packet: {}", addr);
                        let packet = decoder::decode(&buf[..size]).unwrap();
                        OscHandler::handle_packet(packet);
                    }
                    Err(e) => {
                        println!("Error receiving from socket: {}", e);
                    }
                }
            }
        });
    }

    pub fn send_sync(&self, message: OscMessage) {
        let msg_buf3 = encoder::encode(&OscPacket::Message(message)).unwrap();
        self.udp_socket.send_to(&msg_buf3, self.server_address).unwrap();
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

}
