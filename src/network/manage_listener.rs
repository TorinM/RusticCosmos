use pnet::datalink::{self, NetworkInterface, DataLinkReceiver};
use datalink::Channel::Ethernet;

pub fn open_listener() -> Box<dyn DataLinkReceiver> {

    let interface_name = "en0";

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
