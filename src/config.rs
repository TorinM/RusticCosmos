use std::sync::Mutex;

use once_cell::sync::Lazy;

use clap::{Arg, Command};

use crate::output::write_file;

#[derive(Debug)]
pub struct Config {
    pub interface: Option<String>,
    pub output_file_path: Option<String>
}

// Define a global, lazily-initialized CONFIG instance
pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    // Use clap to parse command-line arguments
    let matches = Command::new("rusticcosmos")
        .version("1.0")
        .author("Torin May <torinmay@gmail.com>")
        .about("A network sniffer written for humans to read.")
        .arg(
            Arg::new("interface")
                .short('i')
                .long("interface")
                .value_name("INTERFACE")
                .help("Sets the network interface to use. Required")
                .required(true),
        ).arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("FILTER")
                .help("Sets the packet type(s) to filter to. Options: [tcp, udp, icmp, arp, all]. Include multiple types separated by commas. Optional. Default: all.")
                .required(false),
        ).arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT FILE")
                .help("Sets the output file to write to. Optional. Default: stdout.")
                .required(false),
        )
        .get_matches();

    // Initialize the config based on parsed arguments
    let interface = matches.get_one::<String>("interface").cloned();

    let output_file_path = matches.get_one::<String>("output").and_then(|path| {
        write_file::verify_file_path(path).ok()?;
        write_file::create_file(path).ok()?;
        Some(path.clone())
    });

    Mutex::new(Config {
        interface,
        output_file_path
    })
});
