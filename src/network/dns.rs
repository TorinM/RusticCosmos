use std::net::IpAddr;
use dns_lookup::lookup_addr;

pub fn ip_to_hostname(addr: &IpAddr) -> String {
    match lookup_addr(addr) {
        Ok(host) => host,
        Err(_) => "Unknown".to_string()
    }
}
