use std::env;

// Tool to convert between serial number and engine id formats

/*
- hex encoded engine id: `1B2B0601040181E438010103050F8C9088CDA2A9C8B28180E8CFFF7F` (technically the first byte is the length)
- dotted decimal engine id: `1.3.6.1.4.1.29240.1.1.3.5.15.30021346100634644478317297663`
- hex encoded serial number: `610113512990C8080D13FFFF`
*/

fn convert_objid_hex_to_dotted_decimal(hex: &str) -> String {
    let mut hex = hex.to_string();
    // Remove the first byte (length)
    hex.remove(0);
    hex.remove(0);

    let mut dotted = String::new();
    let mut i = 0;
    let mut largenum : u128 = 0;
    while i < hex.len() {
        let part = &hex[i..i + 2];
        let value = u128::from_str_radix(part, 16).unwrap();

        if i == 0 {
            // Special process on the first byte
            let part0 = value / 40;
            let part1 = value % 40;
            dotted.push_str(&format!("{}.{}", part0, part1));
            i += 2;
            continue;
        }

        // If the value is less than 128, we can just add it to the dotted string
        if value < 128 {
            dotted.push('.');
            if largenum == 0 {
                dotted.push_str(&format!("{}", value));
            } else {
                // If we have a large number, we need to add it to the last part
                let mut temp = largenum << 7;
                temp += value;
                dotted.push_str(&format!("{}", temp));
                largenum = 0;
            }
        } else {
            // If the value is greater than 127, we need to add it to the large number
            if largenum == 0 {
                // If this is the first byte of a large number, we need to set it
                largenum = value & 0x7F;
            } else {
                // Otherwise, we need to shift the large number and add the value
                largenum = (largenum << 7) + (value & 0x7F);
            }
        }

        i += 2;
    }
    return dotted;
}

fn convert_objid_dotted_decimal_to_hex(dotted: &str) -> String {
    let parts: Vec<&str> = dotted.split('.').collect();
    if parts.len() < 2 {
        return String::new();
    }

    let mut hex = String::new();
    for (i, part) in parts.iter().enumerate() {
        let value = part.parse::<u128>().unwrap();
        if i == 0 {
            // Skip
        } else if i == 1 {
            let part0 = parts[0].parse::<u128>().unwrap();
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
    // Known prefix for the engine id
    let prefix = "1.3.6.1.4.1.29240.1.1.3.5.15.";

    // Given a command line parameters in one of the three formats, print all three formats
    let args: Vec<String> = env::args().collect();
    let incoming = &args[1];

    let hex_engine_id;
    let mut dotted_decimal_engine_id = String::new();
    let serial_number;
    // If the incoming string has a dot in it, it is a dotted decimal engine id
    if incoming.contains('.') {
        dotted_decimal_engine_id = incoming.to_string();
        // convert dotted decimal engine id to hex encoded engine id
        hex_engine_id = convert_objid_dotted_decimal_to_hex(&dotted_decimal_engine_id);
        // convert dotted decimal engine id to hex encoded serial number
        let parts: Vec<&str> = dotted_decimal_engine_id.split('.').collect();
        let decimal_serial_number = parts.last().unwrap().parse::<u128>().unwrap();
        serial_number = format!("{:024x}", decimal_serial_number);
    // If the incoming string is 24 bytes long, it is a hex encoded serial number
    } else if incoming.len() == 24 {
        serial_number = incoming.to_string();
        // convert hex encoded serial number to dotted decimal engine id
        let decimal_serial_number = u128::from_str_radix(&serial_number, 16).unwrap();
        dotted_decimal_engine_id.push_str(&format!("{prefix}{decimal_serial_number}"));
        // convert dotted decimal engine id to hex encoded engine id
        hex_engine_id = convert_objid_dotted_decimal_to_hex(&dotted_decimal_engine_id);
    } else {
        hex_engine_id = incoming.to_string();
        // convert hex encoded engine id to dotted decimal engine id
        dotted_decimal_engine_id = convert_objid_hex_to_dotted_decimal(&hex_engine_id);
        // convert dotted decimal engine id to hex encoded serial number
        let parts: Vec<&str> = dotted_decimal_engine_id.split('.').collect();
        let decimal_serial_number = parts.last().unwrap().parse::<u128>().unwrap();
        serial_number = format!("{:024x}", decimal_serial_number);
    }

    println!("Hex encoded engine id: {hex_engine_id}");
    println!("Dotted decimal engine id: {}", dotted_decimal_engine_id);
    println!("Hex encoded serial number: {}", serial_number);
}
