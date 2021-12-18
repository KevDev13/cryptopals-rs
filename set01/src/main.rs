
pub fn chal01() {
    let hs: Vec<u8>; // hex string
    match hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
        Err(_) => panic!("Error decoding"),
        Ok(s) => hs = s,
    }
    let bhs = base64::encode(hs);
    println!("{}", bhs);
}

pub fn chal02() {
    let x1 = "1c0111001f010100061a024b53535009181c";
    let x2 = "686974207468652062756c6c277320657965";
    let x1hex = hex::decode(x1).expect("error decoding x1");
    let x2hex = hex::decode(x2).expect("error decoding x2");
    let mut res: Vec<u8> = Vec::new();
    for c in 0..x1hex.len() {
        res.push(x1hex[c] ^ x2hex[c]);
    }
    println!("{}", hex::encode(res.as_slice()));
}

pub fn chal03() {
}

fn main() {
    chal03();
}
