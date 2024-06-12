use std::env;

use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::DataLinkReceiver;

mod network;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let interface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            Err("USAGE: rusticcosmos <interface>")?
        }
    };
    
    let interface = match network::datalink::get_interface(&interface_name) {
        Ok(i) => i,
        Err(e) => {
            Err(e)?
        }
    };

    let mut rx: Box<dyn DataLinkReceiver> = network::datalink::open_listener(&interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_frame = match EthernetPacket::new(&packet) {
                    Some(p) => p,
                    None => {
                        println!("Failed to parse packet.");
                        continue;
                    }
                };

                let eth = match network::ethernet::handle_ethernet_frame(&interface, &ethernet_frame)
                {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Failed to handle Ethernet frame: {}", e);
                        continue;
                    }
                };

                println!("{:?}", eth);
  
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }
    }
}
