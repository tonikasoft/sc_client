extern crate sc_client;
extern crate env_logger;

use std::env;
use sc_client::{
    DumpOscMode,
    Options, 
    ScClientResult, 
    ScServerErrorMode,
    Server, 
};

fn main() -> ScClientResult<()> {
    env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();

    let options = Options::new("examples/settings.toml");
    let server = Server::new(options);
    server.boot()?;

    server.sync()?;

    server.set_dump_osc_mode(DumpOscMode::PrintParsedAndHex)?;

    server.sync()?;

    server.set_error_mode(ScServerErrorMode::On)?;

    server.sync()?;

    server.get_version(|server_version| {
        println!("{} version is {}.{}{}-{}-{}",
                 server_version.program_name,
                 server_version.major_version,
                 server_version.minor_version, 
                 server_version.patch_name, 
                 server_version.git_branch, 
                 server_version.commit_hash);
    })?;

    server.sync()?;

    server.reboot()?;

    server.sync()?;

    server.set_receive_notifications(true)?;

    server.sync()?;

    server.get_status(|server_status| {
        println!("Number of unit generators: {}\n\
                 Number of synths: {}\n\
                 Number of groups: {}\n\
                 Number of synthdefs: {}\n\
                 Average CPU: {}\n\
                 Peak CPU: {}\n\
                 Nominal sample rate: {}\n\
                 Actual sample rate: {}",
                 server_status.num_of_ugens, 
                 server_status.num_of_synths,
                 server_status.num_of_groups, 
                 server_status.num_of_synthdefs, 
                 server_status.avg_cpu, 
                 server_status.peak_cpu, 
                 server_status.nom_sample_rate, 
                 server_status.actual_sample_rate);
    })?;

    server.sync()?;

    server.shutdown()?;

    Ok(())
}
