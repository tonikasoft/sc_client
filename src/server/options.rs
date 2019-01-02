use config::Config;

pub struct Options<'a> {
    config: &'a Config,
    udp_port_number: u16,
    tcp_port_number: u16,
    verbosity: ServerVerbosity,
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

pub enum ServerVerbosity {
    Normal = 0,
    Shy = -1,
    Introvert = -2,
}

impl<'a> Options<'a> {
    pub fn new(config: &'a Config) -> Self {
        Options {
            config: config,
            udp_port_number: Options::get_udp_port_number_from_config(config),
            tcp_port_number: Options::get_tcp_port_number_from_config(config),
            verbosity: Options::get_server_verbosity_from_config(config),
            num_audio_bus_channels: Options::get_num_audio_bus_channels_from_config(config),
            num_input_bus_channels: Options::get_num_input_bus_channels_from_config(config),
            num_output_bus_channels: Options::get_num_output_bus_channels_from_config(config),
            num_control_bus_channels: Options::get_num_control_bus_channels_from_config(config),
            num_buffers: Options::get_num_buffers_from_config(config),
            max_nodes: Options::get_max_nodes_from_config(config),
            max_synth_defs: Options::get_max_synth_defs_from_config(config),
            load_synth_defs: Options::get_load_synth_defs_from_config(config),
            block_size: Options::get_block_size_from_config(config),
            preferred_hardware_buffer_size: Options::get_preferred_hardware_buffer_size_from_config(config),
            preferred_sample_rate: Options::get_preferred_sample_rate_from_config(config),
            real_time_memory_size: Options::get_real_time_memory_size_from_config(config),
            random_number_generators: Options::get_random_number_generators_from_config(config),
            max_interconnect_buffers: Options::get_max_interconnect_buffers_from_config(config),
            max_logins: Options::get_max_logins_from_config(config),
            session_password: Options::get_session_password_from_config(config),
            device_name: Options::get_device_name_from_config(config),
            input_streams_enable_string: Options::get_input_streams_enable_string_from_config(config),
            output_streams_enable_string: Options::get_output_streams_enable_string_from_config(config),
        }
    }

    fn get_udp_port_number_from_config(config: &Config) -> u16 {
        0
    }

    fn get_tcp_port_number_from_config(config: &Config) -> u16 {
        0
    }

    fn get_server_verbosity_from_config(config: &Config) -> ServerVerbosity {
        ServerVerbosity::Normal
    }

    fn get_num_audio_bus_channels_from_config(config: &Config) -> u16 {
        0
    }

    fn get_num_input_bus_channels_from_config(config: &Config) -> u16 {
        0
    }

    fn get_num_output_bus_channels_from_config(config: &Config) -> u16 {
        0
    }

    fn get_num_control_bus_channels_from_config(config: &Config) -> u16 {
        0
    }

    fn get_num_buffers_from_config(config: &Config) -> u16 {
        0
    }

    fn get_max_nodes_from_config(config: &Config) -> u16 {
        0
    }

    fn get_max_synth_defs_from_config(config: &Config) -> u32 {
        0
    }

    fn get_load_synth_defs_from_config(config: &Config) -> bool {
        true
    }

    fn get_block_size_from_config(config: &Config) -> u16 {
        0
    }

    fn get_preferred_hardware_buffer_size_from_config(config: &Config) -> u16 {
        0
    }

    fn get_preferred_sample_rate_from_config(config: &Config) -> u64 {
        0
    }

    fn get_real_time_memory_size_from_config(config: &Config) -> u64 {
        0
    }
    
    fn get_random_number_generators_from_config(config: &Config) -> u16 {
        0
    }

    fn get_max_interconnect_buffers_from_config(config: &Config) -> u32 {
        0
    }

    fn get_max_logins_from_config(config: &Config) -> u64 {
        0
    }

    fn get_session_password_from_config(config: &Config) -> Option<String> {
        None
    }

    fn get_device_name_from_config(config: &Config) -> Option<String> {
        None
    }

    fn get_input_streams_enable_string_from_config(config: &Config) -> Option<String> {
        None
    }

    fn get_output_streams_enable_string_from_config(config: &Config) -> Option<String> {
        None
    }
}
