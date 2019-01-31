extern crate tokio;
mod tests;
use rosc::{encoder, decoder, OscPacket, OscMessage, OscBundle, OscType};
use self::tokio::io;
use self::tokio::net::UdpSocket as TokioUdpSocket;
use self::tokio::prelude::*;
use std::net::SocketAddr;
use super::{ScClientError, ScClientResult};

pub struct OscServer {
    pub client_address: SocketAddr,
    pub server_address: SocketAddr,
    socket: TokioUdpSocket,
    buf: [u8; rosc::decoder::MTU],
}

impl OscServer {
    //! The addresses are in `ip:port` format.
    pub fn new(client_address: &str, server_address: &str) -> Self {
        let client_addr: SocketAddr = client_address.parse()
            .expect(&format!("Error init client SocketAddr from string {}", client_address));
        let server_addr: SocketAddr = server_address.parse()
            .expect(&format!("Error init server SocketAddr from string {}", server_address));
        let socket = TokioUdpSocket::bind(&client_addr)
            .expect(&format!("Cannot bind UdpSocket to address: {}", client_address));
        let osc_handler = OscServer {
            client_address: client_addr,
            server_address: server_addr,
            socket: socket,
            buf: [0u8; rosc::decoder::MTU],
        };

        osc_handler
    }
}

impl Future for OscServer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let (size, addr) = try_ready!(self.socket.poll_recv_from(&mut self.buf));
            if addr != SocketAddr::from(self.server_address) {
                warn!("Reject packet from unknow host: {}", addr);
                continue;
            }

            let packet = self.parse_packet(&self.buf[..size]).unwrap(); //TODO map_err
            self.handle_packet(&packet).unwrap(); //TODO map_err
        }
    }
}

impl OscServer {
    fn parse_packet(&self, buf: &[u8]) -> ScClientResult<OscPacket> {
        match decoder::decode(buf) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(ScClientError::new(&format!("cannot decode packet: {:?}", e)))
        }
    }

    fn handle_packet(&self, packet: &OscPacket) -> ScClientResult<()> {
        match packet {
            OscPacket::Message(msg) => self.on_message(&msg),
            OscPacket::Bundle(bundle) => self.on_bundle(&bundle)
        }
    }

    fn on_message(&self, message: &OscMessage) -> ScClientResult<()> {
        Ok(println!("{:?}", message))
    }

    fn on_bundle(&self, bundle: &OscBundle) -> ScClientResult<()> {
        Ok(debug!("OSC Bundle: {:?}", bundle))
    }
}
