use hex::*;
use base64:: { encode, decode };

pub fn chal01() {
    let mut hs: Vec<u8>;
    match hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
        Err(_) => panic!("Error decoding"),
        Ok(s) => hs = s,
    }
    let bhs = base64::encode(hs);
    println!("{}", bhs);
}

fn main() {
    chal01();
}
