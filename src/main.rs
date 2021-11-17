mod utils;

fn main() {
    println!("{:?}", utils::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()));
    let c1 = "1c0111001f010100061a024b53535009181c".to_string();
    let c2 = "686974207468652062756c6c277320657965".to_string();
    println!("{:?}", utils::xor_bufs(c1,c2));
}

