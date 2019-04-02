use sc_client::{
    AddAction, DumpOscMode, Options, ScClientResult, Server, Synth, SynthDefinition,
};
use std::env;

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

    SynthDefinition::load(&server, &path_to_synthdef)?;
    server.sync()?;

    Synth::new(&server, synth_name, &AddAction::Tail, -1, &vec![])?;
    rest(2);

    let synth = Synth::new(
        &server,
        synth_name,
        &AddAction::After,
        -1,
        &vec![
            ("amp".into(), 0.1f32.into()).into(),
            ("freq".into(), 440.0f32.into()).into(),
        ],
    )?;
    rest(2);

    let synth_2 = Synth::new(
        &server,
        synth_name,
        &AddAction::After,
        -1,
        &vec![("amp".into(), 0.3f32.into()).into()],
    )?;
    rest(2);

    synth_2.get_control_value(&mut vec!["amp".into(), "att".into()], |value| {
        println!("synth_2's amp: {:?} att: {:?}", value[0], value[1]);
    })?;

    synth_2.get_control_value(&mut vec!["att".into()], |value| {
        println!("synth_2's att: {:?}", value[0]);
    })?;

    synth.get_control_value(&mut vec!["amp".into()], |value| {
        println!("amp value of synth is {:?}", value);
    })?;

    server.sync()?;

    SynthDefinition::free(&server, synth_name)?;
    server.sync()?;

    Ok(())
}

fn rest(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}
