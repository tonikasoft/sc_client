//! Rust client for SuperCollider sound server.
//!
//! ## Configuration
//!
//! Put options inside a configuration file, which you pass to [`Options`](server/struct.Options.html) at initialization.
//! All of the parameters are optional. If a parameter isn't specified, the default value will be used.
//! The next options are available:
//! 
//! | Option                           | Type                 | Default                                       | Description                                                                                                                                                                                                                                                                                                       |
//! | ------                           | ------               | -------                                       | :-----------                                                                                                                                                                                                                                                                                                      |
//! | `address`                        | **String**           | `127.0.0.1`                                   | An IP address.                                                                                                                                                                                                                                                                                                    |
//! | `block_size`                     | **Unsigned Integer** | `64`                                          | The number of samples in one control period.                                                                                                                                                                                                                                                                      |
//! | `client_address`                 | **String**           | `127.0.0.1`                                   |                                                                                                                                                                                                                                                                                                                   |
//! | `client_port`                    | **Unsigned Integer** | `4243`                                        | A port number 0-65535.                                                                                                                                                                                                                                                                                            |
//! | `device_name`                    | **String**           | `None`                                        | Name of the hardware I/O device. If not provided, the system's default device is used.                                                                                                                                                                                                                            |
//! | `input_streams_enable_string`    | **String**           | `None`                                        | Allows turning off input streams that you are not interested in on the device. If the string is 01100, for example, then only the second and third input streams on the device will be enabled. Turning off streams can reduce CPU load.                                                                          |
//! | `load_synth_defs`                | **Boolean**          | `true`                                        | If `false`, then synth definitions will not be loaded on start up.                                                                                                                                                                                                                                                |
//! | `max_interconnect_buffers`       | **Unsigned Integer** | `64`                                          | The maximum number of buffers that are allocated for buffers to interconnect unit generators. Sets the limit of complexity of synth defs that can be loaded at runtime. This value will be increased if a more complex synth-def is loaded at start up time, but it cannot be increased once synthesis has begun. |
//! | `max_logins`                     | **Unsigned Integer** | `64`                                          | Maximum number of named return addresses stored. Also maximum number of tcp connections accepted                                                                                                                                                                                                                  |
//! | `max_nodes`                      | **Unsigned Integer** | `1024`                                        | Maximum number of nodes                                                                                                                                                                                                                                                                                           |
//! | `max_synth_defs`                 | **Unsigned Integer** | `1024`                                        | Maximum number of synth definitions                                                                                                                                                                                                                                                                               |
//! | `num_audio_bus_channels`         | **Unsigned Integer** | `1024`                                        | Number of audio bus channels. The space allocated for audio buses is: (numchannels * (blocksize + 1) * 4)                                                                                                                                                                                                         |
//! | `num_buffers`                    | **Unsigned Integer** | `1024`                                        | Number of sample buffers                                                                                                                                                                                                                                                                                          |
//! | `num_control_bus_channels`       | **Unsigned Integer** | `16384`                                       | Number of control bus channels. The space allocated for control buses is: (numchannels * 8)                                                                                                                                                                                                                       |
//! | `num_input_bus_channels`         | **Unsigned Integer** | `8`                                           | Number of audio input bus channels                                                                                                                                                                                                                                                                                |
//! | `num_of_threads`                 | **Unsigned Integer** | `2`                                           | Number of audio threads.                                                                                                                                                                                                                                                                                          |
//! | `num_output_bus_channels`        | **Unsigned Integer** | `8`                                           | Number of audio output bus channels                                                                                                                                                                                                                                                                               |
//! | `output_streams_enable_string`   | **String**           | `None`                                        | Allows turning off output streams that you are not interested in on the device. If the string is 11000, for example, then only the first two output streams on the device will be enabled. Turning off streams can reduce CPU load.                                                                               |
//! | `preferred_hardware_buffer_size` | **Unsigned Integer** | `0`                                           | If non-zero, it will attempt to set the hardware buffer frame size.                                                                                                                                                                                                                                               |
//! | `preferred_sample_rate`          | **Unsigned Integer** | `0` for `scsynth` and `44100` for `supernova` | If non-zero, it will attempt to set the hardware sample rate.                                                                                                                                                                                                                                                     |
//! | `publish_to_rendezvous`          | **Boolean**          | `false`                                       |                                                                                                                                                                                                                                                                                                                   |
//! | `random_number_generators`       | **Unsigned Integer** | `64`                                          | The number of seedable random number generators.                                                                                                                                                                                                                                                                  |
//! | `real_time_memory_size`          | **Unsigned Integer** | `8192`                                        | The number of kilobytes of real time memory. This memory is used to allocate synths and any memory that unit generators themselves allocate.                                                                                                                                                                      |
//! | `restricted_path`                | **String**           | `None`                                        | If specified, prevents file-accessing OSC commands from accessing files outside `restricted_path`.                                                                                                                                                                                                                |
//! | `session_password`               | **String**           | `None`                                        | When using TCP, the session password must be the first command sent. UDP ports never require passwords, so if password protection is desired, use TCP.                                                                                                                                                            |
//! | `tcp_port_number`                | **Unsigned Integer** | `0`                                           | A port number 0-65535. Only UDP supported. But the server will listen on TCP if you specify this option.                                                                                                                                                                                                          |
//! | `udp_port_number`                | **Unsigned Integer** | `4242`                                        | A port number 0-65535. Only UDP supported.                                                                                                                                                                                                                                                                        |
//! | `ugen_plugins_path`              | **String**           | `None`                                        | A string of paths seperated by `:`. If specified, standard paths are NOT searched for plugins.                                                                                                                                                                                                                    |
//! | `verbosity`                      | **Integer**          | `0`                                           | Controls the verbosity of server messages. A value of 0 is normal behaviour. -1 suppresses informational messages. -2 suppresses informational and many error messages, as well as messages from Poll.                                                                                                            |
//! 
//! > **Note**, `scsynth` has an [issue](https://github.com/supercollider/supercollider/issues/2488) whith setting the same sample rate, which was already set.
//! > The workaround is to use `supernova` or not to set `preferred_sample_rate` for `scsynth` (or set it to `0`). You can set sample rate on your system's settings level.
extern crate config;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rosc;
extern crate chashmap;
#[macro_use] extern crate log;

pub mod server;
pub mod error;
pub use error::ScClientError;
pub use rosc::{ OscType, OscMessage };
