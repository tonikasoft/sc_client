extern crate sc_client;
extern crate env_logger;

use std::env;
use std::fs::File;
use std::io::Read;
use sc_client::{
    AddAction,
    DumpOscMode,
    Options, 
    OscType,
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
    let synth_name = "sc_client_test_1";

    // send buffer
    let mut synthdef_file = File::open(&path_to_synthdef)?;
    let mut buffer = Vec::new();
    synthdef_file.read_to_end(&mut buffer)?;

    SynthDefinition::send(&server, &buffer)?;
    server.sync()?;

    let synth_1 = Synth::new(&server, synth_name, &AddAction::Tail, -1, vec!())?;
    rest(2);
    SynthDefinition::free(&server, synth_name)?;
    server.sync()?;

    // load file
    SynthDefinition::load(&server, &path_to_synthdef)?;
    server.sync()?;

    let synth_2 = Synth::new(
        &server,
        synth_name,
        &AddAction::After,
        synth_1.get_id(),
        vec!(OscType::String("amp".to_string()), OscType::Float(0.1))
    )?;
    rest(2);
    SynthDefinition::free(&server, synth_name)?;
    server.sync()?;

    // load directory
    SynthDefinition::load_directory(&server, "examples/synthdefs")?;
    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::After, synth_2.get_id(), vec!())?;
    rest(2);
    SynthDefinition::free(&server, synth_name)?;
    server.sync()?;

    Ok(())
}

fn rest(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
