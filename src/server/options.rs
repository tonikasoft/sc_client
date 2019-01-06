use config::{Config, ConfigError, File};
use std::path::Path;

#[derive(Serialize, Deserialize)] 
pub struct Options {
    pub bind_to_address: Option<String>,
    pub block_size: u16,
    pub device_name: Option<String>,
    pub input_streams_enable_string: Option<String>,
    pub load_synth_defs: bool,
    pub max_interconnect_buffers: u32,
    pub max_logins: u64,
    pub max_nodes: u16,
    pub max_synth_defs: u32,
    pub num_audio_bus_channels: u16,
    pub num_buffers: u16,
    pub num_control_bus_channels: u16,
    pub num_input_bus_channels: u16,
    pub num_output_bus_channels: u16,
    pub output_streams_enable_string: Option<String>,
    pub path: String,
    pub preferred_hardware_buffer_size: u16,
    pub preferred_sample_rate: u64,
    pub publish_to_rendezvous: bool,
    pub random_number_generators: u16,
    pub real_time_memory_size: u64,
    pub restricted_path: Option<String>,
    pub session_password: Option<String>,
    pub tcp_port_number: u16,
    pub udp_port_number: u16,
    pub ugen_plugins_path: Option<String>,
    pub verbosity: u8,
}

impl Options {
    pub fn new(file_path: &str) -> Self {
        let config = Options::init_config_with_path(file_path);
        let opts = config.try_into::<Options>().unwrap();
        opts.check();
        opts
    }

    fn check(&self) {
        if self.udp_port_number == 0 && self.tcp_port_number == 0 {
            panic!("Either TCP or UDP port should be specified in the configuration file")
        }
    }

    fn init_config_with_path(file_path: &str) -> Config {
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
        Config::try_from::<Options>(&defaults)
            .unwrap()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            bind_to_address: None,
            block_size: 64,
            device_name: None,
            input_streams_enable_string: None,
            load_synth_defs: true,
            max_interconnect_buffers: 64,
            max_logins: 64,
            max_nodes: 1024,
            max_synth_defs: 1024,
            num_audio_bus_channels: 1024,
            num_buffers: 1024,
            num_control_bus_channels: 16384,
            num_input_bus_channels: 8,
            num_output_bus_channels: 8,
            output_streams_enable_string: None,
            path: String::from("/Applications/SuperCollider.app/Contents/Resources/supernova"),
            preferred_hardware_buffer_size: 0,
            preferred_sample_rate: 0,
            publish_to_rendezvous: true,
            random_number_generators: 64,
            real_time_memory_size: 8192,
            restricted_path: None,
            session_password: None,
            tcp_port_number: 4242,
            udp_port_number: 0,
            ugen_plugins_path: None,
            verbosity: 0,
        }
    }
}
