use std::net::Ipv4Addr;
use pnet::datalink::{self, NetworkInterface, DataLinkReceiver};
use pnet::packet::Packet;
use datalink::Channel::Ethernet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ethernet::EthernetPacket;

pub fn open_listener() -> Box<dyn DataLinkReceiver> {

    let interface_name = "en0"; // Preset interface name

    let interface = datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("No network interface found");
    println!("Found interface: {:?}", interface);

    print!("Opening channel...");
    let (_, rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => {
            println!("Connected.");
            (tx, rx)
        },
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    rx
}

pub fn get_source_dest_ips_from_ethernet_packet(eth_packet:&EthernetPacket) -> Result<(Ipv4Addr, Ipv4Addr), String> {
    match eth_packet.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(eth_packet.payload()) {
                let source_ip = ipv4_packet.get_source();
                let destination_ip = ipv4_packet.get_destination();

                Ok((source_ip, destination_ip))
            }
            else {
                Err(String::from("Failed to parse IPv4 packet"))
            }
        },
        pnet::packet::ethernet::EtherTypes::Ipv6 => Err(String::from("Ipv6 not supported")),
        _ => Err(String::from("Unknown ethertype"))
    }
}
