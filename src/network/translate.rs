pub fn packet_payload_to_string(payload: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // let mut payload_string = String::new();

    let payload_string = String::from_utf8_lossy(payload).to_string();

    // for byte in payload {
    //     payload_string.push_str(&format!("{:02x} ", byte));
    // }
    Ok(payload_string)
}