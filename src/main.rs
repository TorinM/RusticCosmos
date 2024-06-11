use pnet::packet::Packet;
use pnet::datalink::DataLinkReceiver;
use pnet::packet::ethernet::EthernetPacket;

mod network;

fn main() {
    let mut listener: Box<dyn DataLinkReceiver> = network::manage_listener::open_listener();
    
    let mut id = 0u128;
    loop {
        match listener.next() {
            Ok(packet) => {
                let eth_packet = match EthernetPacket::new(&packet) {
                    Some(p) => p,
                    None => {
                        println!("Failed to parse packet.");
                        continue;
                    }
                };

                let (source_ip, destination_ip) = match network::manage_listener::get_source_dest_ips_from_ethernet_packet(&eth_packet) {
                    Ok((s,d)) => (s, d),
                    Err(e) => {
                        println!("Failed to get source and destination IPs: {}", e);
                        continue;
                    }
                };
                let payload = eth_packet.payload();

                let packet = network::types::IpPacket::new(id, destination_ip, source_ip, payload.to_vec());
                println!("{}", packet);
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }

        id = id.wrapping_add(1);
    }
}
