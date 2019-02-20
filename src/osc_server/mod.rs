use rosc::{encoder, decoder, OscPacket, OscMessage, OscBundle, OscType};
use std::net::{SocketAddrV4, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::sync::Arc;
use chashmap::CHashMap;
use crate::{ScClientError, ScClientResult};
use std::thread::Thread;

type RespondersMap = CHashMap<String, Box<OscResponder>>;

pub struct OscServer {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
    responders: Arc<RespondersMap>,
    sync_uid: i32,
}

impl OscServer {
    //! The addresses are in `ip:port` format.
    pub fn new(client_address: &str, server_address: &str) -> Self {
        let client_addr = SocketAddrV4::from_str(client_address)
            .expect(&format!("Error init client SocketAddrV4 from string {}", client_address));
        let server_addr = SocketAddrV4::from_str(server_address)
            .expect(&format!("Error init server SocketAddrV4 from string {}", server_address));
        let socket = UdpSocket::bind(client_address)
            .expect(&format!("Cannot bind UdpSocket to address: {}", client_address));
        let responders: RespondersMap = CHashMap::new();
        let mut osc_server = OscServer {
            client_address: client_addr,
            server_address: server_addr,
            udp_socket: Arc::new(socket),
            responders: Arc::new(responders),
            sync_uid: 0,
        };
        osc_server.init_sync_responder();
        osc_server.start_listener();

        osc_server
    }

    fn init_sync_responder(&mut self) {
        // we can't process it in on_message, because we need current thread, which is out 
        // of the on_message context
        let sync_responder = SyncResponder::new();
        self.responders.insert(sync_responder.get_address(), Box::new(sync_responder));
    }

    pub fn sync(&mut self) -> ScClientResult<&Self> {
        self.sync_uid += 1;
        self.send_message("/sync", Some(vec!(OscType::Int(self.sync_uid))))?;
        thread::park();
        Ok(self)
    }

    fn start_listener(&self) {
        let socket = self.udp_socket.clone();
        let server_address = self.server_address.clone();
        let mut responders = self.responders.clone();
        thread::spawn(move || {
            let mut buf = [0u8; rosc::decoder::MTU];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((size, addr)) => OscServer::on_receive_packet(&addr, &buf, size, &server_address, &mut responders)
                        .expect("unexpected OSC error"),
                    Err(e) => error!("Error receiving from socket: {}", e)
                }
            }
        });
    }

    fn on_receive_packet(address: &SocketAddr, buf: &[u8], size: usize, server_address: &SocketAddrV4, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        if *address != SocketAddr::from(*server_address) {
            return Ok(warn!("Reject packet from unknow host: {}", address));
        }

        match decoder::decode(&buf[..size]) {
            Ok(packet) => OscServer::handle_packet(packet, responders),
            Err(e) => Err(ScClientError::new(&format!("cannot decode packet: {:?}", e)))
        }
    }

    fn handle_packet(packet: OscPacket, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        match packet {
            OscPacket::Message(msg) => OscServer::on_message(msg, responders),
            OscPacket::Bundle(bundle) => OscServer::on_bundle(bundle, responders),
        }
    }

    fn on_message(message: OscMessage, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        match message.addr.as_ref() {
            "/done" => OscServer::on_done_message(&message, responders),
            "/fail" => OscServer::on_fail_message(&message),
            _ => OscServer::call_responder_for_key(&message.addr, &message, responders)
        }
    }

    fn on_bundle(bundle: OscBundle, _responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        debug!("OSC Bundle: {:?}", bundle);
        Ok(())
    }

    fn on_done_message(message: &OscMessage, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        debug!("get /done message: {:?}", message);
        match message.args.as_ref() {
            Some(args) => { 
                if let OscType::String(key) = args.clone().remove(0) {
                    return OscServer::call_responder_for_key(&key, message, responders);
                };
                Ok(())
            },
            None => return Ok(debug!("Got /done message, but without any args"))
        }
    }

    fn call_responder_for_key(key: &str, message: &OscMessage, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        let mut response_type = ResponseType::Always;

        if let Some(responder) = responders.get(&key.to_string()) {
            debug!("Calling OSC responder for {}", key);
            response_type = responder.get_response_type();
            responder.callback(message)?;
        };

        if response_type == ResponseType::Once {
            return OscServer::remove_responder_for_key(key, responders);
        }

        Ok(())
    }

    fn remove_responder_for_key(key: &str, responders: &mut Arc<RespondersMap>) -> ScClientResult<()> {
        match responders.remove(&key.to_string()) {
            Some(_) => Ok(info!("responder for key {} with ResponseType::Once has called", key)),
            None => Err(ScClientError::new(&format!("responder for key {} not found", key)))
        }
    }

    fn on_fail_message(message: &OscMessage) -> ScClientResult<()> {
        if let Some(args) = message.args.as_ref() {
            if let OscType::String(addr) = args.clone().remove(0) {
                error!("Server responses with error:\n\t{}, ", addr);
            }
            if let OscType::String(error) = args.clone().remove(1) {
                println!("{}", error);
            };
        };
        Ok(error!("Server responses with /fail message"))
    }

    /// Adds [`OscResponder`](trait.OscResponder.html) to perform on getting message to address.
    pub fn add_responder<T: OscResponder>(&self, responder: T) -> ScClientResult<()> {
        let address = responder.get_address();
        if address == "/synced" {
            return Err(ScClientError::new("can't add responder for reserved address"));
        }

        self.responders.insert(address, Box::new(responder));
        Ok(())
    }

    pub fn remove_responder_for_address(&mut self, address: &str) {
        self.responders.remove(&address.to_string());
    }

    pub fn send_message(&self, address: &str, arguments: Option<Vec<OscType>>) -> ScClientResult<usize> {
        let message = OscMessage {
            addr: address.to_string(),
            args: arguments,
        };
        let msg_buf: Vec<u8> = encoder::encode(&OscPacket::Message(message))
            .map_err(|e| ScClientError::new(&format!("{:?}", e)))?;
        Ok(self.udp_socket.send_to(&msg_buf, self.server_address)
            .map_err(|e| ScClientError::new(&format!("{}", e)))?)
    }
}

struct SyncResponder {
    thread: Thread,
}

impl SyncResponder {
    pub fn new() -> Self {
        SyncResponder {
            thread: thread::current(),
        }
    }
}

impl OscResponder for SyncResponder {
    fn callback(&self, _message: &OscMessage) -> ScClientResult<()> {
        Ok(self.thread.unpark())
    }

    fn get_response_type(&self) -> ResponseType {
        ResponseType::Always
    }

    fn get_address(&self) -> String {
        String::from("/synced")
    }
}

pub trait OscResponder: Send + Sync + 'static {
    fn callback(&self, &OscMessage) -> ScClientResult<()>;
    fn get_response_type(&self) -> ResponseType;
    fn get_address(&self) -> String;
}

#[derive(PartialEq)]
pub enum ResponseType {
    Once,
    Always,
}
