extern crate sc_client;
extern crate env_logger;

use std::env;
use std::fs::File;
use std::io::Read;
use sc_client::{
    DumpOscMode,
    Options, 
    ScClientResult, 
    Server, 
    SynthDefinition,
};

fn main() -> ScClientResult<()> {
    env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();

    let options = Options::new("examples/settings.toml");
    let mut server = Server::new(options);
    server.boot()?;
    server.sync()?;

    server.set_dump_osc_mode(DumpOscMode::PrintParsedAndHex)?;
    server.sync()?;

    let mut synthdef_file = File::open("examples/synthdefs/sc_client_test_1.scsyndef")?;
    let mut buffer = Vec::new();
    synthdef_file.read_to_end(&mut buffer)?;

    {
        let synthdef = SynthDefinition::new(&server);
        synthdef.send(&buffer)?;
    }

    server.sync()?;

    Ok(())
}
