use tokio::signal;
use tokio::sync::mpsc;
use tokio::io::ErrorKind;

use pnet::datalink::DataLinkReceiver;
use pnet::packet::ethernet::EthernetPacket;

mod config;
mod output;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::CONFIG.lock().unwrap();

    let interface_name = config.interface.clone().expect("Unknown error getting user supplied interface name");

    let interface = match network::datalink::get_interface(&interface_name) {
        Ok(i) => i,
        Err(e) => {
            Err(e)?
        }
    };

    let mut datalink_rx: Box<dyn DataLinkReceiver> = network::datalink::open_listener(&interface);

    let (tokio_tx, tokio_rx) = mpsc::channel(32);
    let print_handle = tokio::spawn(output::terminal::print_events(tokio_rx));
    let tokio_tx_clone = tokio_tx.clone();

    let ctrl_c_handle = tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        tokio_tx_clone.send(output::terminal::Event::Exit).await.unwrap();
    });

    loop {
        match datalink_rx.next() {
            Ok(packet) => {
                let ethernet_frame = match EthernetPacket::new(&packet) {
                    Some(p) => p,
                    None => {
                        println!("Failed to parse packet.");
                        continue;
                    }
                };

                let eth = match network::ethernet::handle_ethernet_frame(&interface, &ethernet_frame)
                {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Failed to handle Ethernet frame: {}", e);
                        continue;
                    }
                };

                let message = eth.to_string();
                tokio_tx.send(output::terminal::Event::PrintMessage(message)).await.unwrap();

                // 
                // println!("{}", eth);x
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::Interrupted => {
                        println!("Interrupted.");
                        break;
                    },
                    _ => {
                        println!("An error occurred while reading: {}", e);
                    }
                }
            }
        }
    }

    print!("Cleaning up...");
    ctrl_c_handle.await.unwrap();
    print_handle.await.unwrap();
    println!("Done.");

    Ok(())
}
