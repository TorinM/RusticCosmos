use std::net::{IpAddr, Ipv4Addr};

use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::arp::{ArpPacket, ArpOperations};
use pnet::packet::icmp::{IcmpTypes, IcmpPacket, echo_reply, echo_request};

use serde_derive::{Serialize, Deserialize};

fn handle_icmp(payload: &[u8]) -> (String, u16, u16, Vec<u8>) {
    let icmp_frame: IcmpPacket = IcmpPacket::new(payload).expect("Unable to parse ICMP packet");
    match icmp_frame.get_icmp_type() {
        IcmpTypes::EchoReply => {
            let echo_reply_packet = echo_reply::EchoReplyPacket::new(payload).unwrap();
            let seq = echo_reply_packet.get_sequence_number();
            let id = echo_reply_packet.get_identifier();
            let packet = echo_reply_packet.packet();
            ("EchoReply".to_string(), seq, id, packet.to_vec())
        }
        IcmpTypes::EchoRequest => {
            let echo_request_packet = echo_request::EchoRequestPacket::new(payload).unwrap();
            let seq = echo_request_packet.get_sequence_number();
            let id = echo_request_packet.get_identifier();
            let packet = echo_request_packet.packet();
            ("EchoRequest".to_string(), seq, id, packet.to_vec())
            
        }
        _ => {
            ("Unknown".to_string(), 0, 0, Vec::<u8>::new())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IPv4 {
    pub interface: String,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub destination_ip: IpAddr,
    pub destination_port: u16,
    pub transport_protocol: String,
    pub packet: Vec<u8>,
}
impl IPv4 {
    pub fn new(interface_name: String, ethernet_packet: &EthernetPacket) -> Result<Self, Box<dyn std::error::Error>> {
        let payload = ethernet_packet.payload();

        let header = Ipv4Packet::new(payload).expect(&format!("[{}]: Malformed IPv4 packet", interface_name));

        let source_port;
        let destination_port;
        let transport_protocol;
        let packet;
        match header.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => {
                let tcp_frame = TcpPacket::new(payload).expect("Unable to parse TCP packet");

                transport_protocol = "TCP".to_string();
                source_port = tcp_frame.get_source();
                destination_port = tcp_frame.get_destination();
                packet = tcp_frame.packet().to_vec();
            }
            IpNextHeaderProtocols::Udp => {
                let udp_frame = UdpPacket::new(payload).expect("Unable to parse UDP packet");

                transport_protocol = "UDP".to_string();
                source_port = udp_frame.get_source();
                destination_port = udp_frame.get_destination();
                packet = udp_frame.packet().to_vec();
            }
            IpNextHeaderProtocols::Icmp => {
                let (icmp_type, icmp_seq, icmp_id, icmp_packet) = handle_icmp(payload);
                transport_protocol = format!("ICMP - {}", icmp_type);
                source_port = icmp_seq;
                destination_port = icmp_id;
                packet = icmp_packet;
            }
            _ => {
                transport_protocol = "Unknown".to_string();
                source_port = 0;
                destination_port = 0;
                packet = Vec::<u8>::new();
            }
        }

        Ok(IPv4 {
            interface: interface_name,
            source_ip: IpAddr::V4(header.get_source()),
            source_port: source_port,
            destination_ip: IpAddr::V4(header.get_destination()),
            destination_port: destination_port,
            transport_protocol: transport_protocol,
            packet: packet
        })
    }
}
impl std::fmt::Display for IPv4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) : IPv4-{} : {}:{} -> {}:{}
            \tPacket: {:?}",
            self.interface,
            self.transport_protocol,
            self.source_ip,
            self.source_port,
            self.destination_ip,
            self.destination_port,
            self.packet
        )
    }
}

/*
-------------------------------------------------
*/

#[derive(Serialize, Deserialize)]
pub struct IPv6 {
    pub interface: String,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub destination_ip: IpAddr,
    pub destination_port: u16,
    pub transport_protocol: String,
    pub packet: Vec<u8>,
}
impl IPv6 {
    pub fn new(interface_name: String, ethernet_packet: &EthernetPacket) -> Result<Self, Box<dyn std::error::Error>> {
        let payload = ethernet_packet.payload();

        let header = Ipv6Packet::new(payload).expect(&format!("[{}]: Malformed IPv6 packet", interface_name));

        let source_port;
        let destination_port;
        let transport_protocol;
        let packet;
        match header.get_next_header() {
            IpNextHeaderProtocols::Tcp => {
                let tcp_frame = TcpPacket::new(payload).expect("Unable to parse TCP packet");

                transport_protocol = "TCP".to_string();
                source_port = tcp_frame.get_source();
                destination_port = tcp_frame.get_destination();
                packet = tcp_frame.packet().to_vec();
            }
            IpNextHeaderProtocols::Udp => {
                let udp_frame = UdpPacket::new(payload).expect("Unable to parse UDP packet");

                transport_protocol = "UDP".to_string();
                source_port = udp_frame.get_source();
                destination_port = udp_frame.get_destination();
                packet = udp_frame.packet().to_vec();
            }
            IpNextHeaderProtocols::Icmp => {
                let (icmp_type, icmp_seq, icmp_id, icmp_packet) = handle_icmp(payload);
                transport_protocol = format!("ICMP - {}", icmp_type);
                source_port = icmp_seq;
                destination_port = icmp_id;
                packet = icmp_packet;
            }
            _ => {
                transport_protocol = "Unknown".to_string();
                source_port = 0;
                destination_port = 0;
                packet = Vec::<u8>::new();
            }
        }

        Ok(IPv6 {
            interface: interface_name,
            source_ip: IpAddr::V6(header.get_source()),
            source_port: source_port,
            destination_ip: IpAddr::V6(header.get_destination()),
            destination_port: destination_port,
            transport_protocol: transport_protocol,
            packet: packet
        })
    }
}
impl std::fmt::Display for IPv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) : IPv6-{} : {}:{} -> {}:{}
            \tPacket: {:?}",
            self.interface,
            self.transport_protocol,
            self.source_ip,
            self.source_port,
            self.destination_ip,
            self.destination_port,
            self.packet
        )
    }
}

/*
-------------------------------------------------
*/

#[derive(Serialize, Deserialize)]
pub struct Arp {
    pub interface: String,
    pub source_mac: String,
    pub source_proto_address: Ipv4Addr,
    pub destination_mac: String,
    pub destination_proto_address: Ipv4Addr,
    pub operation: String,
    pub packet: Vec<u8>,
}
impl Arp {
    pub fn new(interface_name: String, ethernet_packet: &EthernetPacket) -> Result<Self, Box<dyn std::error::Error>> {
        let payload = ethernet_packet.payload();

        let header = ArpPacket::new(payload).expect(&format!("[{}]: Malformed Arp packet", interface_name));
        
        let operation = header.get_operation();
        let operation_string = match operation {
            ArpOperations::Request => "Request".to_string(),
            ArpOperations::Reply => "Reply".to_string(),
            _ => "Unknown".to_string(),
        };

        Ok(Arp {
            interface: interface_name,
            source_mac: ethernet_packet.get_source().to_string(),
            source_proto_address: header.get_sender_proto_addr(),
            destination_mac: ethernet_packet.get_destination().to_string(),
            destination_proto_address: header.get_target_proto_addr(),
            operation: operation_string,
            packet: header.payload().to_vec()
        })
    }
}
impl std::fmt::Display for Arp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) : ARP {:?} : {}({}) -> {}({})
            \tPacket: {:?}",
            self.interface,
            self.operation,
            self.source_mac,
            self.source_proto_address,
            self.destination_mac,
            self.destination_proto_address,
            self.packet
        )
    }
}
