use config::{Config, ConfigError, File};
use std::path::Path;

   // -B <bind-to-address>    an IP address
   // -c <number-of-control-bus-channels> (default 16384)
   // -a <number-of-audio-bus-channels>   (default 1024)
   // -i <number-of-input-bus-channels>   (default 8)
   // -o <number-of-output-bus-channels>  (default 8)
   // -z <block-size>                     (default 64)
   // -Z <hardware-buffer-size>           (default 0)
   // -S <hardware-sample-rate>           (default 0)
   // -b <number-of-sample-buffers>       (default 1024)
   // -n <max-number-of-nodes>            (default 1024)
   // -d <max-number-of-synth-defs>       (default 1024)
   // -m <real-time-memory-size>          (default 8192)
   // -w <number-of-wire-buffers>         (default 64)
   // -r <number-of-random-seeds>         (default 64)
   // -D <load synthdefs? 1 or 0>         (default 1)
   // -R <publish to Rendezvous? 1 or 0>  (default 1)
   // -l <max-logins>                     (default 64)
          // maximum number of named return addresses stored
          // also maximum number of tcp connections accepted
   // -p <session-password>
          // When using TCP, the session password must be the first command sent.
          // The default is no password.
          // UDP ports never require passwords, so for security use TCP.
   // -N <cmd-filename> <input-filename> <output-filename> <sample-rate> <header-format> <sample-format>
   // -I <input-streams-enabled>
   // -O <output-streams-enabled>
   // -H <hardware-device-name>
   // -V <verbosity>
          // 0 is normal behaviour.
          // -1 suppresses informational messages.
          // -2 suppresses informational and many error messages, as well as
             // messages from Poll.
          // The default is 0.
   // -U <ugen-plugins-path>
          // A list of paths seperated by `:`.
          // If specified, standard paths are NOT searched for plugins.
   // -P <restricted-path>
          // if specified, prevents file-accessing OSC commands from
          // accessing files outside <restricted-path>.
#[derive(Serialize, Deserialize)] 
pub struct Options {
    /// A path to SuperCollider server.
    pub path: String,
    pub udp_port_number: u16,
    pub tcp_port_number: u16,
    pub verbosity: u8,
    pub num_audio_bus_channels: u16,
    pub num_input_bus_channels: u16,
    pub num_output_bus_channels: u16,
    pub num_control_bus_channels: u16,
    pub num_buffers: u16,
    pub max_nodes: u16,
    pub max_synth_defs: u32,
    pub load_synth_defs: bool,
    pub block_size: u16,
    pub preferred_hardware_buffer_size: u16,
    pub preferred_sample_rate: u64,
    pub real_time_memory_size: u64,
    pub random_number_generators: u16,
    pub max_interconnect_buffers: u32,
    pub max_logins: u64,
    pub session_password: Option<String>,
    pub device_name: Option<String>,
    pub input_streams_enable_string: Option<String>,
    pub output_streams_enable_string: Option<String>,
}

impl Options {
    /// `file_path` - the path to the configuration file.
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
