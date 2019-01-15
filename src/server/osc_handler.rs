use rosc::{encoder, decoder, OscPacket, OscMessage, OscType};
use std::net::{SocketAddrV4, SocketAddr, UdpSocket};
use server::options::Options;
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use chashmap::CHashMap as HashMap;

type Responder = Fn(&OscMessage) + Send + Sync + 'static;
type RespondersMap = HashMap<String, Box<Responder>>;

pub struct OscHandler {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
    responders: Arc<RespondersMap>,
}

impl OscHandler {
    pub fn new(options: &Options) -> Self {
        let client_address = SocketAddrV4::from_str(&format!("{}:{}", options.client_address, options.client_port)).unwrap();
        let server_address = SocketAddrV4::from_str(&format!("{}:{}", options.bind_to_address, options.udp_port_number)).unwrap();
        let socket = UdpSocket::bind(client_address).unwrap();
        let responders: RespondersMap = HashMap::new();
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
        let responders = self.responders.clone();
        thread::spawn(move || {
            let mut buf = [0u8; rosc::decoder::MTU];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((size, addr)) => OscHandler::on_receive_packet(&addr, &buf, size, &server_address, &responders),
                    Err(e) => error!("Error receiving from socket: {}", e)
                }
            }
        });
    }

    fn on_receive_packet(address: &SocketAddr, buf: &[u8], size: usize, server_address: &SocketAddrV4, responders: &Arc<RespondersMap>) {
        if *address != SocketAddr::from(*server_address) {
            return warn!("Reject packet from unknow host: {}", address);
        }

        let packet = decoder::decode(&buf[..size]).unwrap();
        OscHandler::handle_packet(packet, responders);
    }

    fn handle_packet(packet: OscPacket, responders: &Arc<RespondersMap>) {
        match packet {
            OscPacket::Message(msg) => OscHandler::on_message(msg, responders),
            OscPacket::Bundle(bundle) => debug!("OSC Bundle: {:?}", bundle)
        }
    }

    fn on_message(message: OscMessage, responders: &Arc<RespondersMap>) {
        match message.addr.as_ref() {
            "/done" => OscHandler::on_done_message(&message, responders),
            "/fail" => OscHandler::on_fail_message(&message),
            _ => OscHandler::call_responder_for_key(&message.addr, &message, responders)
        }
    }

    fn on_done_message(message: &OscMessage, responders: &Arc<RespondersMap>) {
        match message.args.as_ref() {
            Some(args) => { 
                if let OscType::String(key) = args.clone().remove(0) {
                    OscHandler::call_responder_for_key(&key, message, responders)
                }
            },
            None => debug!("Got /done message, but without any args")
        }
    }

    fn call_responder_for_key(key: &str, message: &OscMessage, responders: &Arc<RespondersMap>) {
        if let Some(callback) = responders.get(&key.to_string()) {
            debug!("Calling OSC responder for {}", key);
            callback(message)
        }
    }

    fn on_fail_message(message: &OscMessage) {
        if let Some(args) = message.args.as_ref() {
            if let OscType::String(addr) = args.clone().remove(0) {
                error!("Server responses with error:\n\t{}, ", addr);
            }
            if let OscType::String(error) = args.clone().remove(1) {
                println!("{}", error);
            }
        } else {
            error!("Server responses with /fail message");
        }
    }

    pub fn add_responder_for_address<F: Fn(&OscMessage) + Send + Sync + 'static>(&mut self, address: &str, callback: F) {
        self.responders.insert(String::from(address), Box::new(callback));
    }

    pub fn remove_responder_for_address(&mut self, address: &str) {
        self.responders.remove(&address.to_string());
    }

    pub fn send_message(&self, message: OscMessage) {
        let msg_buf3 = encoder::encode(&OscPacket::Message(message)).unwrap();
        self.udp_socket.send_to(&msg_buf3, self.server_address).unwrap();
    }

}
