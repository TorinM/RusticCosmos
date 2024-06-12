use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};

use crate::network::types;

#[derive(Debug)]
pub enum EthernetFrame {
    IPv4(types::IPv4),
    IPv6(types::IPv6),
    Arp(types::Arp),
    Unknown,
}

pub fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) -> Result<EthernetFrame, Box<dyn std::error::Error>> {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            match types::IPv4::new(interface_name.to_string(), ethernet)
            {
                Ok(ipv4) => Ok(EthernetFrame::IPv4(ipv4)),
                Err(e) => {
                    println!("[{}]: Malformed IPv4 packet: {}", interface_name, e);
                    return Err(e);
                }
            }
        },
        EtherTypes::Ipv6 => {
            match types::IPv6::new(interface_name.to_string(), ethernet)
            {
                Ok(ipv6) => Ok(EthernetFrame::IPv6(ipv6)),
                Err(e) => {
                    println!("[{}]: Malformed IPv6 packet: {}", interface_name, e);
                    return Err(e);
                }
            }
        },
        EtherTypes::Arp => {
            match types::Arp::new(interface_name.to_string(), ethernet)
            {
                Ok(arp) => Ok(EthernetFrame::Arp(arp)),
                Err(e) => {
                    println!("[{}]: Malformed ARP packet: {}", interface_name, e);
                    return Err(e);
                }
            }
        },
        _ => {
            println!("[{}]: Unknown packet type: {:?}", interface_name, ethernet.get_ethertype());
            Ok(EthernetFrame::Unknown)
        }
    }
}
