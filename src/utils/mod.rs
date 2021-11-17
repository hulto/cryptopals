extern crate base64;
use std::char;


pub fn hex_to_ascii_string(hex_string: String) -> String {
    let mut ascii_string = "".to_string();
    for i in (0..hex_string.chars().count()-1).step_by(2) {
        // Convert hex -> u8 with from_str_radix.
        // Unwrap the Result and cast to String.
        let string_rep = u8::from_str_radix(&hex_string[i..i+2], 16).map(|n| n as char).unwrap().to_string();
        ascii_string = format!("{}{}", ascii_string, string_rep);
    }
    return ascii_string;
}

#[test]
pub fn test_hex_to_ascii_string() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let res_str = "I'm killing your brain like a poisonous mushroom".to_string();
    assert_eq!(hex_to_ascii_string(hex_str), res_str);
}






pub fn ascii_to_b64_string(ascii_string: String) -> String {
    // println!("{:?}", ascii_string.as_bytes());
    // println!("{:?}", b"I'm killing your brain like a poisonous mushroom");
    return base64::encode(ascii_string.as_bytes());
}

#[test]
pub fn test_ascii_to_b64_string() {
    let ascii_str = "I'm killing your brain like a poisonous mushroom".to_string();
    let res_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(ascii_to_b64_string(ascii_str), res_str);
}






pub fn hex_to_base64(hex_str: String) -> String {
    let ascii_string = hex_to_ascii_string(hex_str);
    let base64_string = ascii_to_b64_string(ascii_string);
    return base64_string;
}

#[test]
pub fn test_s1c1_hex_to_base64() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let res_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(hex_str), res_str);
}

