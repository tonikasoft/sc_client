use rosc::{encoder, decoder, OscPacket, OscMessage, OscType};
use std::net::{SocketAddrV4, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use chashmap::CHashMap as HashMap;
use super::super::ScClientError;

type Responder = Fn(&OscMessage) + Send + Sync + 'static;
type RespondersMap = HashMap<String, Box<Responder>>;

pub struct OscHandler {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
    responders: Arc<RespondersMap>,
}

impl OscHandler {
    pub fn new(client_address: &str, server_address: &str) -> Self {
        let client_addr = SocketAddrV4::from_str(client_address)
            .expect(&format!("Error init client SocketAddrV4 from string {}", client_address));
        let server_addr = SocketAddrV4::from_str(server_address)
            .expect(&format!("Error init server SocketAddrV4 from string {}", server_address));
        let socket = UdpSocket::bind(client_address)
            .expect(&format!("Cannot bind UdpSocket to address: {}", client_address));
        let responders: RespondersMap = HashMap::new();
        let osc_handler = OscHandler {
            client_address: client_addr,
            server_address: server_addr,
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

        match decoder::decode(&buf[..size]) {
            Ok(packet) => OscHandler::handle_packet(packet, responders),
            Err(e) => error!("cannot decode packet: {:?}", e)
        }
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

    pub fn send_message(&self, address: &str, arguments: Option<Vec<OscType>>) -> Result<(), ScClientError> {
        let message = OscMessage {
            addr: address.to_string(),
            args: arguments,
        };
        let msg_buf: Vec<u8> = encoder::encode(&OscPacket::Message(message))
            .map_err(|e| ScClientError::OSC(format!("{:?}", e)))?;
        self.udp_socket.send_to(&msg_buf, self.server_address)
            .map_err(|e| ScClientError::OSC(format!("{}", e)))?;
        Ok(())
    }

}