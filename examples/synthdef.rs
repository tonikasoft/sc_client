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
    let server = Server::new(options);
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

    let synthdef = SynthDefinition::new(&server);
    synthdef.send(&buffer)?;

    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::Tail, -1, vec!())?;
    rest(2);
    synthdef.free(synth_name)?;
    server.sync()?;

    // load file
    synthdef.load(&path_to_synthdef)?;
    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::After, -1, vec!())?;
    rest(2);

    synthdef.free(synth_name)?;
    server.sync()?;

    // load directory
    synthdef.load_directory("examples/synthdefs")?;
    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::After, -1, vec!())?;
    rest(2);
    synthdef.free(synth_name)?;
    server.sync()?;

    Ok(())
}

fn rest(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
