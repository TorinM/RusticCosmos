use std::error::Error;
use std::fs::File;

use crate::network::ethernet::EthernetFrame;

pub fn write_ethernet_frame_to_file(path: &String, data: &EthernetFrame) -> Result<(), Box<dyn Error>> {
    // Open a file in write-only mode
    let file = File::open(path)?;

    let mut wtr = csv::Writer::from_writer(file);

    wtr.serialize(data)?;
    wtr.flush()?;
    Ok(())
}

pub fn create_file(path: &String) -> Result<(), Box<dyn Error>> {
    File::create(path)?;
    Ok(())
}

pub fn verify_file_path(path: &String) -> Result<(), Box<dyn Error>> {
    if std::path::Path::new(path).exists() {
        return Err("File already exists".into());
    }
    Ok(())
}
