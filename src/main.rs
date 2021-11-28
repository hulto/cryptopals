mod utils;

fn main() {
    let cipher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    println!("{:?}", utils::single_char_xor_brute_force(&cipher_text));

}

