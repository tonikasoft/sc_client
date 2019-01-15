use config::{Config, ConfigError, File};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)] 
pub struct Options {
    pub address: String,
    pub block_size: u16,
    pub client_address: String,
    pub client_port: u16,
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
    pub num_of_threads: u8,
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
        let conf: Config;
        match Options::init_config_with_path(file_path) {
            Ok(c) => conf = c,
            Err(e) => conf = Options::on_error_reading_config(e)
        }

        let opts = conf.try_into::<Options>()
            .expect("Unable to convert config into Options struct");
        opts.check();
        opts
    }

    fn check(&self) {
        if self.udp_port_number == 0 && self.tcp_port_number == 0 {
            panic!("Either TCP or UDP port should be specified in the configuration file")
        }
    }

    fn init_config_with_path(file_path: &str) -> Result<Config, ConfigError> {
        let mut config = Config::new();
        let config_file = File::from(Path::new(file_path));
        match config.merge(config_file) {
            Ok(conf) => Ok(Options::set_config_defaults(conf))?,
            Err(e) => Err(e)
        }
    }

    fn set_config_defaults(config: &mut Config) -> Result<Config, ConfigError> {
        let defaults = Options::default();
        Ok(config
           .set_default("address", defaults.address)?
           .set_default("block_size", defaults.block_size as i64)?
           .set_default("client_address", defaults.client_address)?
           .set_default("client_port", defaults.client_port as i64)?
           .set_default("device_name", defaults.device_name)?
           .set_default("input_streams_enable_string", defaults.input_streams_enable_string)?
           .set_default("load_synth_defs", defaults.load_synth_defs)?
           .set_default("max_interconnect_buffers", defaults.max_interconnect_buffers as i64)?
           .set_default("max_logins", defaults.max_logins as i64)?
           .set_default("max_nodes", defaults.max_nodes as i64)?
           .set_default("max_synth_defs", defaults.max_synth_defs as i64)?
           .set_default("num_audio_bus_channels", defaults.num_audio_bus_channels as i64)?
           .set_default("num_buffers", defaults.num_buffers as i64)?
           .set_default("num_control_bus_channels", defaults.num_control_bus_channels as i64)?
           .set_default("num_input_bus_channels", defaults.num_input_bus_channels as i64)?
           .set_default("num_of_threads", defaults.num_of_threads as i64)?
           .set_default("num_output_bus_channels", defaults.num_output_bus_channels as i64)?
           .set_default("output_streams_enable_string", defaults.output_streams_enable_string)?
           .set_default("path", defaults.path)?
           .set_default("preferred_hardware_buffer_size", defaults.preferred_hardware_buffer_size as i64)?
           .set_default("preferred_sample_rate", defaults.preferred_sample_rate as i64)?
           .set_default("publish_to_rendezvous", defaults.publish_to_rendezvous)?
           .set_default("random_number_generators", defaults.random_number_generators as i64)?
           .set_default("real_time_memory_size", defaults.real_time_memory_size as i64)?
           .set_default("restricted_path", defaults.restricted_path)?
           .set_default("session_password", defaults.session_password)?
           .set_default("tcp_port_number", defaults.tcp_port_number as i64)?
           .set_default("udp_port_number", defaults.udp_port_number as i64)?
           .set_default("ugen_plugins_path", defaults.ugen_plugins_path)?
           .set_default("verbosity", defaults.verbosity as i64)?
           .to_owned()
           )
    }

    fn on_error_reading_config(e: ConfigError) -> Config {
        info!("{}.\nUsing default configuration.", e);
        let defaults = Options::default();
        Config::try_from::<Options>(&defaults)
            .expect("Cannot init config from default Options")
    }

    pub fn to_args(&self) -> Vec<String> {
        let result = vec!(
            Options::get_arg_with_value_or_empty_vec("-H", self.device_name.clone()),
            Options::get_arg_with_value_or_empty_vec("-I", self.input_streams_enable_string.clone()),
            Options::get_arg_with_value_or_empty_vec("-O", self.output_streams_enable_string.clone()),
            Options::get_arg_with_value_or_empty_vec("-P", self.restricted_path.clone()),
            Options::get_arg_with_value_or_empty_vec("-U", self.ugen_plugins_path.clone()),
            Options::get_arg_with_value_or_empty_vec("-p", self.session_password.clone()),
            vec!(String::from("-D"), (self.load_synth_defs as i32).to_string()),
            vec!(String::from("-R"), (self.publish_to_rendezvous as i32).to_string()),
            vec!(String::from("-S"), self.preferred_sample_rate.to_string()),
            vec!(String::from("-T"), self.num_of_threads.to_string()),
            vec!(String::from("-V"), self.verbosity.to_string()),
            vec!(String::from("-Z"), self.preferred_hardware_buffer_size.to_string()),
            vec!(String::from("-a"), self.num_audio_bus_channels.to_string()),
            vec!(String::from("-b"), self.num_buffers.to_string()),
            vec!(String::from("-c"), self.num_control_bus_channels.to_string()),
            vec!(String::from("-d"), self.max_synth_defs.to_string()),
            vec!(String::from("-i"), self.num_input_bus_channels.to_string()),
            vec!(String::from("-l"), self.max_logins.to_string()),
            vec!(String::from("-m"), self.real_time_memory_size.to_string()),
            vec!(String::from("-n"), self.max_nodes.to_string()),
            vec!(String::from("-o"), self.num_output_bus_channels.to_string()),
            vec!(String::from("-r"), self.random_number_generators.to_string()),
            vec!(String::from("-t"), self.tcp_port_number.to_string()),
            vec!(String::from("-u"), self.udp_port_number.to_string()),
            vec!(String::from("-w"), self.max_interconnect_buffers.to_string()),
            vec!(String::from("-z"), self.block_size.to_string()),
            );

        result.into_iter()
            .flatten()
            .collect()
    }

    fn get_arg_with_value_or_empty_vec(arg: &str, value: Option<String>) -> Vec<String> {
        if let Some(val) = value {
            return vec!(String::from(arg), val)
        }
        vec!()
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            address: String::from("127.0.0.1"),
            block_size: 64,
            client_address: String::from("127.0.0.1"),
            client_port: 4243,
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
            num_of_threads: 2,
            num_output_bus_channels: 8,
            output_streams_enable_string: None,
            path: String::from("/Applications/SuperCollider.app/Contents/Resources/supernova"),
            preferred_hardware_buffer_size: 0,
            preferred_sample_rate: 44100,
            publish_to_rendezvous: false,
            random_number_generators: 64,
            real_time_memory_size: 8192,
            restricted_path: None,
            session_password: None,
            tcp_port_number: 0,
            udp_port_number: 4242,
            ugen_plugins_path: None,
            verbosity: 0,
        }
    }
}
