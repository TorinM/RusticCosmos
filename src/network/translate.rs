use std::string::FromUtf8Error;

pub fn packet_payload_to_string(payload: Vec<u8>) -> Result<String, FromUtf8Error> {
    // let mut payload_string = String::new();

    String::from_utf8(payload)

    // for byte in payload {
    //     payload_string.push_str(&format!("{:02x} ", byte));
    // }
    // Ok(payload_string)
}