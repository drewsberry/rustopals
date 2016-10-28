use std::collections::HashMap;

#[cfg(test)]
mod set1 {
    #[test]
    fn challenge1() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let bytes = hex_string_to_bytes(hex_string);
        let base64 = bytes_to_base64(bytes);
    }

    fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
        let mut bytes_vec = Vec::new();
        let hex_chars = hex_string.chars();

        loop {
            let hex_pair = (hex_chars.next(), hex_chars.next());

            match pair {
                (Some(first_char), Some(second_char)) => {
                    let first_byte = hex_char_to_byte(first_char);
                    let second_byte = hex_char_to_byte(second_char);

                    let combined_byte = first_byte << 4 | second_byte;
                    bytes_vec.push(combined_byte);
                },
                (Some(first_char), None) => panic!("Hex strings must come in pairs."),
                (None, None) => break;
            }
        }

        bytes_vec
    }

    fn hex_char_to_byte(hex_char: char) -> u8 {
        match hex_char {
            '0' .. '9' => return hex_char as u8 - '0' as u8 + 0x00,
            'a' .. 'z' => return hex_char as u8 - 'a' as u8 + 0x0a,
            'A' .. 'Z' => return hex_char as u8 - 'A' as u8 + 0x0a,
            _ => panic!("Character {} is not valid hex.", hex_char),
        }
    }

    const BASE_64: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'X', 'Y', 'Z', 'a', 'b', 'c',
        'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '+', '/',
    ];

    fn bytes_to_base64(bytes: Vec<u8>) -> String {
        let mut base64 = String::new();

        loop {
            let quadruple = ()
            base64.push(new_char);
        }
    }
}
