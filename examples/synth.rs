extern crate sc_client;
extern crate env_logger;

use std::env;
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

    SynthDefinition::load(&server, &path_to_synthdef)?;
    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::Tail, -1, vec!())?;
    rest(2);

    let synth = Synth::new(
        &server,
        synth_name,
        &AddAction::After,
        -1,
        vec!(OscType::String("amp".to_string()), OscType::Float(0.1))
    )?;
    rest(2);

    synth.get_control_value(&server, OscType::String("amp".to_string()), |value| {
        println!("amp value is {:?}", value);
    })?;

    server.sync()?;

    SynthDefinition::free(&server, synth_name)?;
    server.sync()?;

    Ok(())
}

fn rest(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
