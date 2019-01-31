extern crate sc_client;
extern crate env_logger;

use sc_client::{Server, Options, DumpOscMode};
use std::thread;
use std::time::Duration;
use std::env;
use sc_client::ScClientError;

fn main() -> Result<(), ScClientError> {
    env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();

    let options = Options::new("examples/settings.toml");
    let mut server = Server::new(options);
    server.boot();

    thread::sleep(Duration::from_secs(5));

    server.set_dump_osc_mode(DumpOscMode::PrintParsedAndHex)?;

    server.sync();
    server.get_version(|name, major, minor, patch_n, branch, hash| {
        println!("{} version is {}.{}{}-{}-{}", name, major, minor, patch_n, branch, hash);
    })?;

    server.sync();

    server.reboot()?;

    thread::sleep(Duration::from_secs(5));

    server.set_receive_notifications(true)?;
    server.sync();
    server.get_status(|num_of_ugens, num_of_synths, num_of_groups, num_of_synthdefs, avg_cpu, peak_cpu, nom_sr, sr| {
        println!("Number of unit generators: {}\n\
                 Number of synths: {}\n\
                 Number of groups: {}\n\
                 Number of synthdefs: {}\n\
                 Average CPU: {}\n\
                 Peak CPU: {}\n\
                 Nominal sample rate: {}\n\
                 Actual sample rate: {}",
                 num_of_ugens, num_of_synths, num_of_groups, num_of_synthdefs, avg_cpu, peak_cpu, nom_sr, sr);
    })?;

    loop{std::thread::sleep(Duration::from_millis(1))}
}
