use rosc::{encoder, decoder, OscPacket, OscMessage};
use std::net::{ SocketAddrV4, SocketAddr, UdpSocket };
use server::options::Options;
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;

pub struct OscHandler {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
    responders: Arc<HashMap<String, Box<Fn(OscPacket) + Send + Sync + 'static>>>,
}

impl OscHandler {
    pub fn new(options: &Options) -> Self {
        let client_address = SocketAddrV4::from_str(&format!("{}:{}", options.client_address, options.client_port)).unwrap();
        let server_address = SocketAddrV4::from_str(&format!("{}:{}", options.bind_to_address, options.udp_port_number)).unwrap();
        let socket = UdpSocket::bind(client_address).unwrap();
        let responders: HashMap<String, Box<Fn(OscPacket) + Send + Sync + 'static>> = HashMap::new();
        let osc_handler = OscHandler {
            client_address: client_address,
            server_address: server_address,
            udp_socket: Arc::new(socket),
            responders: Arc::new(responders),
        };
        osc_handler.start_listener();

        osc_handler
    }

    fn start_listener(&self) {
        let socket = self.udp_socket.clone();
        let server_address = self.server_address.clone();
        thread::spawn(move || {
            let mut buf = [0u8; rosc::decoder::MTU];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((size, addr)) => OscHandler::on_receive_packet(&addr, &buf, size, &server_address),
                    Err(e) => println!("Error receiving from socket: {}", e)
                }
            }
        });
    }

    fn on_receive_packet(address: &SocketAddr, buf: &[u8], size: usize, server_address: &SocketAddrV4) {
        if *address != SocketAddr::from(*server_address) {
            return println!("Reject packet from unknow host: {}", address);
        }

        let packet = decoder::decode(&buf[..size]).unwrap();
        OscHandler::handle_packet(packet);
    }

    fn handle_packet(packet: OscPacket) {
        match packet {
            OscPacket::Message(msg) => OscHandler::on_message(msg),
            OscPacket::Bundle(bundle) => println!("OSC Bundle: {:?}", bundle)
        }
    }

    fn on_message(message: OscMessage) {
        match message.addr.as_ref() {
            "/done" => OscHandler::on_done_message(&message),
            "/fail" => OscHandler::on_fail_message(&message),
            _ => println!("Receive OSC message {}", message.addr)
        }
        match message.args {
            Some(args) => println!("OSC arguments: {:?}", args),
            None => println!("No arguments in message."),
        }
    }

    fn on_done_message(message: &OscMessage) {
        match message.args.as_ref() {
            Some(args) => {
                let address = args[0].clone();
            },
            None => println!("Get /done message, but without args")
        }
    }

    fn on_fail_message(message: &OscMessage) {

    }

    pub fn add_responder_for_address<F: Fn(OscPacket) + Send + Sync + 'static>(&mut self, address: &str, callback: F) {
        (*Arc::get_mut(&mut self.responders).unwrap()).insert(String::from(address), Box::new(callback));
    }

    pub fn remove_responder_for_address(&mut self, address: &str) {
        (*Arc::get_mut(&mut self.responders).unwrap()).remove(address);
    }

    pub fn send_message(&self, message: OscMessage) {
        let msg_buf3 = encoder::encode(&OscPacket::Message(message)).unwrap();
        self.udp_socket.send_to(&msg_buf3, self.server_address).unwrap();
    }

}
