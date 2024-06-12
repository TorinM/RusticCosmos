use tokio::sync::mpsc;

#[derive(Debug)]
pub enum Event {
    PrintMessage(String),
    Exit,
}

const HELP_MSG: &str = "Use ctrl+c at any time to exit.";

pub async fn print_events(mut rx: mpsc::Receiver<Event>) {
    while let Some(event) = rx.recv().await {
        match event {
            Event::PrintMessage(message) => {
                // Print the message
                print!("{}[2J", 27 as char); // Clear the terminal
            
                println!("{}", HELP_MSG);
                println!("-------------------------------");
                println!("{}", message);
                println!("-------------------------------");
            },
            Event::Exit => {
                println!("Received Exit Message");
                break;
            }
        }
    }
}