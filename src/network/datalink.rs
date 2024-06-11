use pnet::util::MacAddr;

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface, DataLinkReceiver};

use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};

pub fn get_interface(interface_name:&str) -> Result<NetworkInterface, String> {
    match datalink::interfaces()
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next() {
            Some(interface) => Ok(interface),
            None => Err(format!("Interface {} not found.", interface_name))
    }
}

pub fn open_listener(interface: &NetworkInterface) -> Box<dyn DataLinkReceiver> {
    print!("Opening channel...");
    let (_, rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => {
            println!("Connected.");
            (tx, rx)
        },
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    rx
}

pub fn generate_fake_ethernet_frame<'a>(packet: &[u8], buf: &'a mut[u8], interface: &NetworkInterface) -> Result<MutableEthernetPacket<'a>, String> {
    let mut payload_offset = 0;
    let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();

    if interface.is_loopback(){
        payload_offset = 14;
    }
    match Ipv4Packet::new(&packet[payload_offset..]).unwrap().get_version() {
        4 => {
            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
            fake_ethernet_frame.set_payload(&packet[payload_offset..]);
            Ok(fake_ethernet_frame)
        },
        6 => {
            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
            fake_ethernet_frame.set_payload(&packet[payload_offset..]);
            Ok(fake_ethernet_frame)
        }
        _ => {
            Err("Unknown packet version.".to_string())
        }
    }
}
