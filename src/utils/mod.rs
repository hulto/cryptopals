extern crate base64;
use std::char;


pub fn single_char_xor_brute_force(cipher_text: &String) -> String {
    let mut key_text = "".to_string();
    let cipher_len = cipher_text.chars().count()/2;
    for key in 0..256 {
        key_text = "".to_string();
        for i in 0..cipher_len {
            key_text = format!("{}{:x}", key_text, key);
        }
        // println!("key: {:?}", key_text);
        // println!("cip: {:?}", cipher_text);
        let xor_res = xor_bufs(&key_text, &cipher_text);
        let string_res = hex_to_ascii_string(&xor_res);
        if test_readable_string(&string_res) {
            println!("{:?}", string_res);
        }
        // println!("xor: {:?}", xor_res);
    }
    return "".to_string();
}

#[test]
pub fn test_s1c3_xor_single_byte_brute_force() {
    let c1 = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    let res = "Cooking MC's like a pound of bacon".to_string();
    assert_eq!(single_char_xor_brute_force(&c1), res);
}






pub fn test_readable_string(test_string: &String) -> bool{
    for c in test_string.chars() {
        if ! c.is_ascii() {
            return false
        }
    }
    return true;
}

#[test]
pub fn test_test_readable_string() {
    let c1 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*(){}',.;:/?=+\"<>-_|\\".to_string();
    let c2 = "å".to_string();
    assert!(test_readable_string(&c1));
    assert!(!test_readable_string(&c2));
}





pub fn xor_bufs(c1: &String, c2: &String) -> String {
    let mut res_string = "".to_string();
    // Instead of to string do to int with two step hex iteration.
    for i in (0..c1.chars().count()).step_by(2) {
        let i1:i32 = i32::from_str_radix(&c1[i..i+2], 16).unwrap();
        let i2:i32 = i32::from_str_radix(&c2[i..i+2], 16).unwrap();
        let xor_res = i1 ^ i2;
        res_string = format!("{}{:x}", res_string, xor_res)
    }
    return res_string;
}

#[test]
pub fn test_s1c2_xor_bufs() {
    let c1 = "1c0111001f010100061a024b53535009181c".to_string();
    let c2 = "686974207468652062756c6c277320657965".to_string();
    let res = "746865206b696420646f6e277420706c6179".to_string();
    assert_eq!(xor_bufs(&c1,&c2), res);
}




pub fn hex_to_ascii_string(hex_string: &String) -> String {
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
    assert_eq!(hex_to_ascii_string(&hex_str), res_str);
}







pub fn ascii_to_b64_string(ascii_string: &String) -> String {
    // println!("{:?}", ascii_string.as_bytes());
    // println!("{:?}", b"I'm killing your brain like a poisonous mushroom");
    return base64::encode(ascii_string.as_bytes());
}

#[test]
pub fn test_ascii_to_b64_string() {
    let ascii_str = "I'm killing your brain like a poisonous mushroom".to_string();
    let res_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(ascii_to_b64_string(&ascii_str), res_str);
}






pub fn hex_to_base64(hex_str: &String) -> String {
    let ascii_string = hex_to_ascii_string(hex_str);
    let base64_string = ascii_to_b64_string(&ascii_string);
    return base64_string;
}

#[test]
pub fn test_s1c1_hex_to_base64() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let res_str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(&hex_str), res_str);
}
