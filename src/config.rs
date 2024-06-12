use clap::{Arg, Command};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Config {
    pub interface: String,
    pub output_file: String
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
                .help("Sets the network interface to use")
                .required(true),
        ).arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT FILE")
                .help("Sets the output file to write to, defaults to output.pcap")
                .required(false),
        )
        .get_matches();

    // Initialize the config based on parsed arguments
    let interface = match matches.get_one::<String>("interface") {
        Some(i) => i.clone(),
        None => {
            eprintln!("No interface provided.");
            std::process::exit(1);
        }
    };

    let output_file = match matches.get_one::<String>("output") {
        Some(o) => o.clone(),
        None => "output.pcap".to_string()
    };

    Mutex::new(Config {
        interface,
        output_file
    })
});
