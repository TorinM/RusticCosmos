use pnet::datalink::NetworkInterface;

use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use crate::network::types;

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => types::IPv4::new(interface_name.to_string(), ethernet),
        EtherTypes::Ipv6 => types::IPv6::new(interface_name.to_string(), ethernet),
        EtherTypes::Arp => types::Arp::new(interface_name.to_string(), ethernet),
        _ => {
            println!("[{}]: Unknown packet type: {:?}", interface_name, ethernet.get_ethertype());
        }
    }
}
