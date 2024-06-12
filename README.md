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
sudo rusticcosmos [OPTIONS] --interface <INTERFACE>
```

### Options

```bash
Options:
  -i, --interface <INTERFACE>  Sets the network interface to use
  -o, --output <OUTPUT FILE>   Sets the output file to write to, defaults to output.pcap
  -h, --help                   Print help
  -V, --version                Print version
```

## Dependencies

- [pnet](https://crates.io/crates/pnet) - Rust library for low-level networking using __libpnet__
- [clap](https://crates.io/crates/clap) - Command line argument parser for Rust
- [dns-lookup](https://crates.io/crates/dns-lookup) - Rust library for DNS lookups
- [once_cell](https://crates.io/crates/once_cell) - Rust library for lazy statics
