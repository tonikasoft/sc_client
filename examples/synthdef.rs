extern crate sc_client;
extern crate env_logger;

use std::env;
use std::fs::File;
use std::io::Read;
use sc_client::{
    AddAction,
    DumpOscMode,
    Options, 
    ScClientResult, 
    Server, 
    Synth,
    SynthDefinition,
};

fn main() -> ScClientResult<()> {
    env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();

    let options = Options::new("examples/settings.toml");
    let mut server = Server::new(options);
    server.boot()?;
    server.sync()?;

    server.set_dump_osc_mode(DumpOscMode::PrintParsed)?;
    server.sync()?;

    let path_to_synthdef = "examples/synthdefs/sc_client_test_1.scsyndef";
    let mut synthdef_file = File::open(&path_to_synthdef)?;
    let mut buffer = Vec::new();
    synthdef_file.read_to_end(&mut buffer)?;

    // send buffer
    SynthDefinition::send(&server, &buffer)?;
    server.sync()?;

    Synth::new(&server, "sc_client_test_1", &AddAction::Tail, -1)?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    SynthDefinition::free(&server, "sc_client_test_1")?;
    server.sync()?;

    // load from file
    SynthDefinition::load(&server, &path_to_synthdef)?;
    server.sync()?;

    Synth::new(&server, "sc_client_test_1", &AddAction::After, -1)?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    SynthDefinition::free(&server, "sc_client_test_1")?;
    server.sync()?;

    // load directory
    SynthDefinition::load_directory(&server, "examples/synthdefs")?;
    server.sync()?;

    Synth::new(&server, "sc_client_test_1", &AddAction::After, -1)?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    SynthDefinition::free(&server, "sc_client_test_1")?;
    server.sync()?;

    Ok(())
}
