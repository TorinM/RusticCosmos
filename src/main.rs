use pnet::datalink::DataLinkReceiver;
use pnet::packet::ethernet::EthernetPacket;

mod network;

fn main() {
    let mut listener: Box<dyn DataLinkReceiver> = network::manage_listener::open_listener();
    
    loop {
        match listener.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(&packet).unwrap();
                println!("Received new packet: {:?}", ethernet_packet);
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
        }
    }
}
