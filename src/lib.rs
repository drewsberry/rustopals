#[cfg(test)]
mod set1 {
    #[test]
    fn challenge1() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let bytes = hex_string_to_bytes(hex_string);
        let base64 = bytes_to_base64(bytes);

        println!("Expected: {}", base64_string);
        println!("Result:   {}", base64);

        assert!(base64 == base64_string, "Unexpected result.");
    }

    // Target: XOR two hex strings.
    #[test]
    fn challenge2() {
        let first_hex = "1c0111001f010100061a024b53535009181c";
        let second_hex = "686974207468652062756c6c277320657965";
        let expected_result = "746865206b696420646f6e277420706c6179";

        let first_bytes = hex_string_to_bytes(first_hex);
        let second_bytes = hex_string_to_bytes(second_hex);

        let xor_result: Vec<u8> = first_bytes
            .iter()
            .zip(second_bytes.iter())
            .map(|byte_pair| byte_pair.0 ^ byte_pair.1)
            .collect();

        let xor_string = bytes_to_hex_string(xor_result);

        println!("Expected: {}", expected_result);
        println!("Result:   {}", xor_string);

        assert!(xor_string == expected_result, "Unexpected result.");
    }

    fn hex_string_to_bytes(hex_string: &str) -> Vec<u8> {
        let mut bytes_vec = Vec::new();
        let mut hex_chars = hex_string.chars();

        loop {
            let hex_pair = (hex_chars.next(), hex_chars.next());

            match hex_pair {
                (Some(first_char), Some(second_char)) => {
                    let first_byte = hex_char_to_byte(first_char);
                    let second_byte = hex_char_to_byte(second_char);

                    let combined_byte = first_byte << 4 | second_byte;
                    bytes_vec.push(combined_byte);
                }
                (Some(_), None) => panic!("Hex strings must come in pairs."),
                (None, None) => break,
                _ => panic!(),
            }
        }

        bytes_vec
    }

    fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
        let mut hex_string = String::new();

        for byte in bytes {
            let (first_char, second_char) = byte_to_hex_tuple(byte);
            hex_string.push(first_char);
            hex_string.push(second_char);
        }

        hex_string
    }

    fn hex_char_to_byte(hex_char: char) -> u8 {
        match hex_char {
            '0'...'9' => return hex_char as u8 - '0' as u8 + 0x00,
            'a'...'z' => return hex_char as u8 - 'a' as u8 + 0x0a,
            'A'...'Z' => return hex_char as u8 - 'A' as u8 + 0x0a,
            _ => panic!("Character {} is not valid hex.", hex_char),
        }
    }

    fn byte_to_hex_tuple(byte: u8) -> (char, char) {
        let first_byte = byte >> 4;
        let second_byte = byte & 0x0f;

        let first_char = half_byte_to_hex_char(first_byte);
        let second_char = half_byte_to_hex_char(second_byte);

        (first_char, second_char)
    }

    fn half_byte_to_hex_char(half_byte: u8) -> char {
        match half_byte {
            0x0...0x9 => return (half_byte - 0x0 + '0' as u8) as char,
            0xa...0xf => return (half_byte - 0xa + 'a' as u8) as char,
            _ => panic!(),
        }
    }

    const BASE_64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                                 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                                 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                                 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                                 '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

    fn bytes_to_base64(mut bytes: Vec<u8>) -> String {
        let mut base64 = String::new();
        bytes.reverse();

        loop {
            match (bytes.pop(), bytes.pop(), bytes.pop()) {
                (Some(first), Some(second), Some(third)) => {
                    let new_chars = byte_triplet_to_base64_quadruplet(first, second, third);
                    for new_char in new_chars.iter() {
                        base64.push(*new_char);
                    }
                }
                (Some(first), Some(second), None) => {
                    println!("Two instead of three...");
                    let new_chars = byte_triplet_to_base64_quadruplet(first, second, 0);
                    for new_char in new_chars.iter() {
                        base64.push(*new_char);
                    }
                    break;
                }
                (Some(first), None, None) => {
                    println!("One instead of three...");
                    let new_chars = byte_triplet_to_base64_quadruplet(first, 0, 0);
                    for new_char in new_chars.iter() {
                        base64.push(*new_char);
                    }
                    break;
                }
                (None, None, None) => {
                    break;
                }
                _ => panic!(),
            }
        }

        base64
    }

    fn byte_triplet_to_base64_quadruplet(first: u8, second: u8, third: u8) -> [char; 4] {
        let byte_quadruplet = [first >> 2,
                               (first & 0b00_00_00_11) << 4 | second >> 4,
                               (second & 0b00_00_11_11) << 2 | third >> 6,
                               third & 0b00_11_11_11];

        let base64_quadruplet = [BASE_64[byte_quadruplet[0] as usize],
                                 BASE_64[byte_quadruplet[1] as usize],
                                 BASE_64[byte_quadruplet[2] as usize],
                                 BASE_64[byte_quadruplet[3] as usize]];

        base64_quadruplet
    }
}
