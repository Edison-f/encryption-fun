use std::io::{stdin, stdout, Write};

pub(crate) fn rc4_runner() {
    println!("Enter your key (Read as hex, padded to even len w/ 0 if needed)");
    let _ = stdout().flush();
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Problem reading string");
    if let Some('\n') = key.chars().next_back() {
        key.pop();
    }
    if let Some('\r') = key.chars().next_back() {
        key.pop();
    }
    if key.len() % 2 != 0 {
        key.push('0');
    }
    
    
    println!("Enter your plaintext");
    let _ = stdout().flush();
    let mut plaintext = String::new();
    stdin().read_line(&mut plaintext).expect("Problem reading string");
    if let Some('\n') = plaintext.chars().next_back() {
        plaintext.pop();
    }
    if let Some('\r') = plaintext.chars().next_back() {
        plaintext.pop();
    }

    println!("Enter # of discarded keystream bytes");
    let _ = stdout().flush();
    let mut discard = String::new();
    stdin().read_line(&mut discard).expect("Problem reading string");
    if let Some('\n') = discard.chars().next_back() {
        discard.pop();
    }
    if let Some('\r') = discard.chars().next_back() {
        discard.pop();
    }
    let key = generate_keystream(key, discard, plaintext.as_bytes().len());
    println!("Ciphertext:\n");
    for i in 0..plaintext.as_bytes().len() {
        print!("{}", &format!("{:#04x}", plaintext.as_bytes()[i] ^ key[i])[2..4].to_uppercase());
    }
}

pub(crate) fn generate_keystream(key: String, discard: String, text_len: usize) -> Vec<u8> {
    let discard = u64::from_str_radix(&*discard, 10).unwrap_or_else(|_| {
        println!("Discard amount automatically set to 0");
        0
    });
    let mut i = 0;
    let mut key_bytes: Vec<u8> = Vec::new();
    while i < key.len() {
        let byte = u8::from_str_radix(&key[i..(i + 2)], 16);
        if let Ok(byte) = byte {
            key_bytes.push(byte);
        } else {
            key_bytes.push(0x77);
            println!("Malformed hex, 0x77 pushed instead")
        }
        i += 2;
    }
    let mut s = Vec::new();
    let mut k = Vec::new();
    let mut i = 0;
    while i < 256 {
        s.push(i as u8);
        k.push(key_bytes[i % key_bytes.len()]);
        i += 1;
    }
    let mut j = 0;
    let mut i = 0;
    while i < 256 {
        j = (j + s[i] as u16 + k[i] as u16) % 256;//k[i] as u16) % 256;
        s.swap(i, j as usize);
        i += 1;
    }
    // Init done
    let mut i = 0;
    let mut j = 0;

    // Keystream start
    let mut keystream = Vec::new();
    for count in 0..=(discard + text_len as u64) {
        i = (i + 1) % 256;
        j = (j + s[i % 256] as u16) % 256;
        s.swap(i, j as usize);
        if count >= discard {
            let t: u8 = ((s[i % 256] as u16 + s[j as usize] as u16) % 256) as u8;
            keystream.push(s[t as usize]);
        }
    }
    return keystream;
}