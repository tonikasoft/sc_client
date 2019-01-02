use config::{Config, File};

#[derive(Deserialize)]
pub struct Options {
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
    pub fn new(file_name: &str) -> Self {
        let config = Options::init_config_with_name(file_name);
        let opts = config.deserialize::<Options>().unwrap();
        opts.check();
        opts
    }

    fn check(&self) {
        if self.udp_port_number == 0 && self.tcp_port_number == 0 {
            panic!("Either TCP or UDP port should be specified in the configuration file")
        }
    }

    fn init_config_with_name(file_name: &str) -> Config {
        let mut config = Config::new();
        let config_file = File::with_name(file_name);
        config
            .merge(config_file)
            .expect(&format!("Error reading {}.toml", file_name))
            .to_owned()
    }
}
