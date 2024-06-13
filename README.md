# Rustic Cosmos

A command line tool to sniff packets on a specified network interface. Rustic Cosmos translates raw packets into a human-readable format.

## Installation

Rustic Cosmos is available on [crates.io](https://crates.io/crates/rusticcosmos). You can install it using cargo:

```bash
cargo install rusticcosmos
```

## Usage

Rustic Cosmos requires _root_ privileges to run and monitor network traffic. You can run it using the following command:

### Command

```bash
sudo rusticcosmos [OPTIONS] --interface <INTERFACE> --filter <FILTER>
```

### Options

```bash
Options:
  -i, --interface <INTERFACE>  Sets the network interface to use. Required
  -f, --filter <FILTER>        Sets the packet type(s) to filter to. Options: [tcp, udp, icmp, arp, all]. Include multiple types separated by commas. Optional. Default: all
  -o, --output <OUTPUT FILE>   Sets the output file to write to. Optional
  -h, --help                   Print help
  -V, --version                Print version
```

## Dependencies

- [pnet](https://crates.io/crates/pnet) - Rust library for low-level networking using __libpnet__
- [tokio](https://crates.io/crates/tokio) - Asynchronous runtime for Rust
- [clap](https://crates.io/crates/clap) - Command line argument parser for Rust
- [dns-lookup](https://crates.io/crates/dns-lookup) - Rust library for DNS lookups
- [once_cell](https://crates.io/crates/once_cell) - Rust library for lazy statics
- [serde](https://crates.io/crates/serde) - Rust library for serializing and deserializing data structures
