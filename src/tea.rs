use std::io::{stdin, stdout, Write};

// TODO: Add variable round count, could also use wrapping to make things nicer

pub(crate) fn tea_runner() {
    println!("You really don't want to rely on this, TEA implementations seem to vary");
    println!("Enter your key (Truncated/padded to 16 chars with 'NUL' chars)");
    let _ = stdout().flush();
    let mut key = String::new();
    stdin().read_line(&mut key).expect("Problem reading string");
    if let Some('\n') = key.chars().next_back() {
        key.pop();
    }
    if let Some('\r') = key.chars().next_back() {
        key.pop();
    }
    while key.len() < 16 {
        key.push(0 as char);
    }
    if key.len() > 16 {
        key = key[0..16].to_string();
    }
    println!("{}", key.len());

    println!("Enter your plaintext (length padded by 'NUL' to be divisible by 4)");
    let _ = stdout().flush();
    let mut plaintext = String::new();
    stdin().read_line(&mut plaintext).expect("Problem reading string");
    if let Some('\n') = plaintext.chars().next_back() {
        plaintext.pop();
    }
    if let Some('\r') = plaintext.chars().next_back() {
        plaintext.pop();
    }
    while plaintext.len() % 8 != 0 {
        plaintext.push(0 as char);
    }
    tea_setup(key, plaintext);
}

pub(crate) fn tea_setup(key: String, plaintext: String) {
    let mut split_key = Vec::new();
    let mut i = 0;
    while i < key.len() {
        split_key.push(key[i..i + 4].parse::<u32>().unwrap_or(0));
        i += 4;
    }

    let mut split_plaintext = Vec::new();
    i = 0;
    while i < plaintext.len() {
        split_plaintext.push((plaintext[i..i + 4].parse::<u32>().unwrap_or(0), // might be off by one
                              plaintext[i + 5..i + 8].parse::<u32>().unwrap_or(0)));
        i += 8;
    }
    tea_encrypt(split_key, split_plaintext);
}

pub(crate) fn tea_encrypt(key: Vec<u32>, plaintext: Vec<(u32, u32)>) {
    let delta: u32 = 0x9e3779b9;
    let modulus = u64::pow(2, 32);
    print!("0x");
    for pair in plaintext {
        let mut sum = 0;
        let mut left = pair.0 as u64;
        let mut right = pair.1 as u64;
        for _ in 1..=32 {
            sum = (sum + delta as u64) % modulus;
            left = (left
                + ((((right << 4) + key[0] as u64) % modulus)
                ^ ((right + sum) % modulus) ^ (((right >> 5) + key[1] as u64) % modulus))) % modulus;
            right = (right
                + ((((left << 4) + key[2] as u64) % modulus)
                ^ ((left + sum) % modulus) ^ (((left >> 5) + key[3] as u64) % modulus))) % modulus;
        }
        for byte in Vec::from((left as u32).to_be_bytes()) {
            print!("{}", format!("{:#04x}", byte)[2..4].to_string())
        }
        for byte in Vec::from((right as u32).to_be_bytes()) {
            print!("{}", format!("{:#04x}", byte)[2..4].to_string())
        }
    }
}
