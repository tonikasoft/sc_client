extern crate env_logger;
#[allow(unused_imports)] use super::*;

#[test]
fn on_receive_packet() {
    // let osc_handler = OscHandler::new("127.0.0.1:4243", "127.0.0.1:4242");
}

#[test]
fn osc_server() {
    setup();
    let osc_server = OscServer::new("127.0.0.1:4243", "127.0.0.1:57120");
    tokio::run(osc_server.map_err(|e| println!("server error = {:?}", e)));
}

#[allow(dead_code)]
fn setup() {
    std::env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();
}
