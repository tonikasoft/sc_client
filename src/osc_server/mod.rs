use rosc::{encoder, decoder, OscPacket, OscMessage, OscBundle, OscType};
use std::net::{SocketAddrV4, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::sync::{Arc, RwLock};
use crate::{ScClientError, ScClientResult};
use std::thread::Thread;

type Responders = RwLock<Vec<Box<OscResponder>>>;

pub struct OscServer {
    pub client_address: SocketAddrV4,
    pub server_address: SocketAddrV4,
    udp_socket: Arc<UdpSocket>,
    responders: Arc<Responders>,
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
        let mut osc_server = OscServer {
            client_address: client_addr,
            server_address: server_addr,
            udp_socket: Arc::new(socket),
            responders: Arc::new(RwLock::new(Vec::new())),
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
        self.responders.write()
            .expect("can't write responder")
            .push(Box::new(sync_responder));
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

    fn on_receive_packet(address: &SocketAddr, buf: &[u8], size: usize, server_address: &SocketAddrV4, responders: &mut Arc<Responders>) -> ScClientResult<()> {
        if *address != SocketAddr::from(*server_address) {
            return Ok(warn!("Reject packet from unknow host: {}", address));
        }

        match decoder::decode(&buf[..size]) {
            Ok(packet) => OscServer::handle_packet(packet, responders),
            Err(e) => Err(ScClientError::new(&format!("cannot decode packet: {:?}", e)))
        }
    }

    fn handle_packet(packet: OscPacket, responders: &mut Arc<Responders>) -> ScClientResult<()> {
        match packet {
            OscPacket::Message(msg) => OscServer::on_message(msg, responders),
            OscPacket::Bundle(bundle) => OscServer::on_bundle(bundle, responders),
        }
    }

    fn on_message(message: OscMessage, responders: &mut Arc<Responders>) -> ScClientResult<()> {
        match message.addr.as_ref() {
            "/done" => OscServer::on_done_message(&message, responders),
            "/fail" => OscServer::on_fail_message(&message),
            _ => OscServer::call_responders_for_key(&message.addr, &message, responders)
        }
    }

    fn on_bundle(bundle: OscBundle, _responders: &mut Arc<Responders>) -> ScClientResult<()> {
        debug!("OSC Bundle: {:?}", bundle);
        Ok(())
    }

    fn on_done_message(message: &OscMessage, responders: &mut Arc<Responders>) -> ScClientResult<()> {
        debug!("get /done message: {:?}", message);
        match message.args.as_ref() {
            Some(args) => { 
                if let OscType::String(key) = args.clone().remove(0) {
                    return OscServer::call_responders_for_key(&key, message, responders);
                };
                Ok(())
            },
            None => return Ok(debug!("Got /done message, but without any args"))
        }
    }

    fn call_responders_for_key(key: &str, message: &OscMessage, responders: &mut Arc<Responders>) -> ScClientResult<()> {
        responders.write()
            .map_err(|e| ScClientError::new(&format!("{}", e)))?
            .retain(|ref responder| {
                if responder.get_address() == key {
                    responder.callback(message).unwrap();
                    return responder.get_after_call_action(message) == AfterCallAction::Reschedule;
                }
                true
            });

        Ok(())
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
        if responder.get_address() == "/synced" {
            return Err(ScClientError::new("can't add responder for reserved address"));
        }

        Ok(self.responders
           .write()
           .map_err(|e| ScClientError::new(&format!("{}", e)))?
           .push(Box::new(responder)))
    }

    pub fn remove_responders_for_address(&mut self, address: &str) -> ScClientResult<()> {
        Ok(self.responders
           .write()
           .map_err(|e| ScClientError::new(&format!("{}", e)))?
           .retain(|ref responder| {
               responder.get_address() != address.to_string()
           }))
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

    fn get_after_call_action(&self, _message: &OscMessage) -> AfterCallAction {
        AfterCallAction::Reschedule
    }

    fn get_address(&self) -> String {
        String::from("/synced")
    }
}

pub trait OscResponder: Send + Sync + 'static {
    fn callback(&self, &OscMessage) -> ScClientResult<()>;
    fn get_after_call_action(&self, &OscMessage) -> AfterCallAction;
    fn get_address(&self) -> String;
}

#[derive(PartialEq)]
pub enum AfterCallAction {
    None,
    Reschedule,
}
