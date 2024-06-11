use std::net::Ipv4Addr;
use dns_lookup::lookup_addr;

pub struct IpPacket {
    pub id: u128,
    pub destination: Ipv4Addr,
    pub source: Ipv4Addr,
    pub payload: Vec<u8>,
    pub destination_hostname: String
}
impl std::fmt::Display for IpPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IP Packet {}: {} -> {}({}) ({} bytes)", self.id, self.source, self.destination_hostname, self.destination, self.payload.len())
    }
}
impl IpPacket {
    pub fn new(id: u128, destination: Ipv4Addr, source: Ipv4Addr, payload: Vec<u8>) -> IpPacket {
        let ipaddr = std::net::IpAddr::V4(destination);
        let host_name = match lookup_addr(&ipaddr) {
            Ok(host) => host,
            Err(_) => "Unknown".to_string()
        };

        IpPacket {
            id,
            destination,
            source,
            payload,
            destination_hostname: host_name
        }
    }
}
