use std::env;

// Tool to convert between serial number and engine id formats

/*
- hex encoded engine id: `1B2B0601040181E438010103050F8C9088CDA2A9C8B28180E8CFFF7F` (technically the first byte is the length)
- dotted decimal engine id: `1.3.6.1.4.1.29240.1.1.3.5.15.30021346100634644478317297663`
- hex encoded serial number: `610113512990C8080D13FFFF`
*/

fn convert_objid_dotted_decimal_to_hex(dotted: &str) -> String {
    let parts: Vec<&str> = dotted.split('.').collect();
    if parts.len() < 2 {
        return String::new();
    }

    let mut hex = String::new();
    for (i, part) in parts.iter().enumerate() {
        // u64::MAX is 20 digits long: 18446744073709551615
        if part.len() > 19 {
            println!("Unhandled part: {part}");
            continue;
        }
        let value = part.parse::<u64>().unwrap();
        if i == 0 {
            // Skip
        } else if i == 1 {
            let part0 = parts[0].parse::<u64>().unwrap();
            hex.push_str(&format!("{:02x}", part0 * 40 + value));
        } else {
            if value < 128 {
                hex.push_str(&format!("{:02x}", value));
            } else {
                // least significant byte
                let mut largenum = String::new();
                largenum.push_str(&format!("{:02x}", value & 0x7F));

                // rest of the bytes
                let mut value = value >> 7;
                while value > 0 {
                    let mut b = (value & 0x7F) as u8;
                    b |= 0x80;
                    let hex = format!("{:02x}", b);
                    largenum.insert_str(0, &hex);
                    value = value >> 7;
                }
                hex.push_str(&largenum);
            }
        }
    }
    let length = format!("{:02x}", hex.len() / 2);
    hex.insert_str(0, &length);

    return hex;
}

// hex encoded engine id <-> dotted decimal engine id <-> hex encoded serial number
fn main() {
    // Given a command line parameters in one of the three formats, print all three formats
    let args: Vec<String> = env::args().collect();
    let incoming = &args[1];
    let mut hex_engine_id = String::new();
    let mut dotted_decimal_engine_id = String::new();
    let mut serial_number = String::new();
    // If the incoming string has a dot in it, it is a dotted decimal engine id
    if incoming.contains('.') {
        dotted_decimal_engine_id = incoming.to_string();
        // convert dotted decimal engine id to hex encoded engine id
        hex_engine_id = convert_objid_dotted_decimal_to_hex(&dotted_decimal_engine_id);
        // let decimal_serial_number = parts.last();
        //let serial_number = format!("{:x}", decimal_serial_number);
    // If the incoming string is 24 bytes long, it is a hex encoded serial number
    } else if incoming.len() == 24 {
        serial_number = incoming.to_string();
        // convert hex encoded serial number to dotted decimal engine id
    } else {
        hex_engine_id = incoming.to_string();
        // convert hex encoded engine id to serial number
    }


    println!("Hex encoded engine id: {}", hex_engine_id);
    println!("Dotted decimal engine id: {}", dotted_decimal_engine_id);
    println!("Hex encoded serial number: {}", serial_number);
}
