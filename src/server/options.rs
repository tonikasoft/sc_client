use config::{Config, ConfigError, File};
use std::path::Path;
use std::error::Error;

#[derive(Serialize, Deserialize)] 
pub struct Options {
    /// A path to SuperCollider server.
    pub path: String,
    udp_port_number: u16,
    tcp_port_number: u16,
    verbosity: u8,
    num_audio_bus_channels: u16,
    num_input_bus_channels: u16,
    num_output_bus_channels: u16,
    num_control_bus_channels: u16,
    num_buffers: u16,
    max_nodes: u16,
    max_synth_defs: u32,
    load_synth_defs: bool,
    block_size: u16,
    preferred_hardware_buffer_size: u16,
    preferred_sample_rate: u64,
    real_time_memory_size: u64,
    random_number_generators: u16,
    max_interconnect_buffers: u32,
    max_logins: u64,
    session_password: Option<String>,
    device_name: Option<String>,
    input_streams_enable_string: Option<String>,
    output_streams_enable_string: Option<String>,
}

impl Options {
    /// `file_path` - the path to the configuration file.
    pub fn new(file_path: &str) -> Self {
        let config = Options::init_config_with_name(file_path);
        let opts = config.try_into::<Options>().unwrap();
        opts.check();
        opts
    }

    fn check(&self) {
        if self.udp_port_number == 0 && self.tcp_port_number == 0 {
            panic!("Either TCP or UDP port should be specified in the configuration file")
        }
    }

    fn init_config_with_name(file_path: &str) -> Config {
        let mut config = Config::new();
        let config_file = File::from(Path::new(file_path));
        match config.merge(config_file) {
            Ok(conf) => conf.to_owned(),
            Err(e) => Options::on_error_reading_config(e)
        }
    }

    fn on_error_reading_config(e: ConfigError) -> Config {
        println!("{}.\nUsing default configuration.", e);
        let defaults = Options::default();
        let config = Config::try_from::<Options>(&defaults)
            .unwrap();
        config
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            path: String::from("/Applications/SuperCollider.app/Contents/Resources/supernova"),
            udp_port_number: 0,
            tcp_port_number: 4242,
            verbosity: 0,
            num_audio_bus_channels: 1024,
            num_input_bus_channels: 8,
            num_output_bus_channels: 8,
            num_control_bus_channels: 16384,
            num_buffers: 1024,
            max_nodes: 1024,
            max_synth_defs: 1024,
            load_synth_defs: true,
            block_size: 64,
            preferred_hardware_buffer_size: 512,
            preferred_sample_rate: 44100,
            real_time_memory_size: 8192,
            random_number_generators: 64,
            max_interconnect_buffers: 64,
            max_logins: 64,
            session_password: None,
            device_name: None,
            input_streams_enable_string: None,
            output_streams_enable_string: None,
        }
    }
}
