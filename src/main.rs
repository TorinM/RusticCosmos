use std::env;
use pnet::packet::ethernet::EthernetPacket;

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
                let ethernet_frame;
                if cfg!(target_os = "macos") // Workaround for macOS to make a fake Ethernet frame
                    && interface.is_broadcast()
                    && !interface.is_broadcast()
                    && ((!interface.is_loopback() && interface.is_point_to_point()) || interface.is_loopback())
                {
                    let mut buf: [u8; 1600] = [0u8; 1600];
                    ethernet_frame = network::datalink::generate_fake_ethernet_frame(&packet, &mut buf, &interface).unwrap().to_immutable();
                }
                else {
                    ethernet_frame = match EthernetPacket::new(&packet) {
                        Some(p) => p,
                        None => {
                            println!("Failed to parse packet.");
                            continue;
                        }
                    };
                }
                
                // let (source_ip, destination_ip) = match network::datalink::get_source_dest_ips_from_ethernet_frame(&eth_packet) {
                //     Ok((s,d)) => (s, d),
                //     Err(e) => {
                //         println!("Failed to get source and destination IPs: {}", e);
                //         continue;
                //     }
                // };
                // let payload = eth_packet.payload();

                // let packet = network::types::IpPacket::new(id, destination_ip, source_ip, payload.to_vec());
                // println!("{}", packet);
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }
    }
}
