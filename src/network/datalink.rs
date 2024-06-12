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

pub fn generate_fake_ethernet_frame(packet: &[u8], interface: &NetworkInterface) -> Result<Option<(Vec<u8>, MutableEthernetPacket<'static>)>, String> {
    if cfg!(target_os = "macos") // Workaround for macOS to make a fake Ethernet frame
        && interface.is_broadcast()
        && !interface.is_broadcast()
        && ((!interface.is_loopback() && interface.is_point_to_point()) || interface.is_loopback())
    {
        let buf = vec![0; 1600];
        let mut payload_offset = 0;
    
        let mut fake_ethernet_frame = MutableEthernetPacket::owned(buf.clone()).unwrap();
    
        if interface.is_loopback(){
            payload_offset = 14;
        }
        match Ipv4Packet::new(&packet[payload_offset..]).unwrap().get_version() {
            4 => {
                fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
                fake_ethernet_frame.set_payload(&packet[payload_offset..]);
                Ok(Some((buf, fake_ethernet_frame)))
            },
            6 => {
                fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
                fake_ethernet_frame.set_payload(&packet[payload_offset..]);
                Ok(Some((buf, fake_ethernet_frame)))
            }
            _ => {
                Err("Unknown IP version".to_string())
            }
        }
    }
    else {
        Ok(None)
    }
    
}
