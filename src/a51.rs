use std::io::{stdin, stdout, Write};

// Resources used:
// https://asecuritysite.com/symmetric/a5
// http://koclab.cs.ucsb.edu/teaching/cren/project/2017/jensen+andersen.pdf

// This could be a variant of A5/1 or original A5/1, I'm not sure

pub(crate) fn a51_runner() {
    println!("Enter your key as hex (Truncated/padded to 64 bits/8 bytes with 0)");
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
        key += "0";
    }
    if key.len() > 16 {
        key = key[0..16].to_string();
    }
    let key = u64::from_str_radix(key.as_str(), 16).unwrap_or(0);

    println!("Enter your frame number as a decimal number from 0-4194303");
    let _ = stdout().flush();
    let mut frame = String::new();
    stdin().read_line(&mut frame).expect("Problem reading string");
    if let Some('\n') = frame.chars().next_back() {
        frame.pop();
    }
    if let Some('\r') = frame.chars().next_back() {
        frame.pop();
    }
    let mut frame = frame.parse::<u32>().unwrap_or(0);//u32::from_str_radix(frame.as_str(), 10).unwrap_or(0);
    if frame > 4194303 {
        frame = 0;
        println!("Frame number out of bounds, set to zero")
    }

    println!("Enter your plaintext/ciphertext, start with 0x for hex mode");
    let _ = stdout().flush();
    let mut plaintext = String::new();
    stdin().read_line(&mut plaintext).expect("Problem reading string");
    if let Some('\n') = plaintext.chars().next_back() {
        plaintext.pop();
    }
    if let Some('\r') = plaintext.chars().next_back() {
        plaintext.pop();
    }
    let plaintext = if &plaintext[0..2] == "0x" {
        if plaintext.len() % 2 != 0 {
            plaintext += "0";
        }
        let mut i = 2;
        let mut res = Vec::new();
        while i < plaintext.len() {
            res.push(u8::from_str_radix(&plaintext[i..i + 2], 16).unwrap_or(0));
            i += 2;
        }
        res
    } else {
        plaintext.into_bytes()
    };

    let regs = setup(key, frame);
    let keystream = generate_keystream(regs, plaintext.len());
    for byte in &keystream {
        println!("{}", byte)
    }
    for i in 0..keystream.len() {
        print!("{}", &format!("{:#04x}", plaintext[i] ^ keystream[i])[2..4].to_uppercase());
    }
}

fn setup(key: u64, frame: u32) -> Vec<u64> {
    let feedback_locs = vec![vec![13, 16, 17, 18], vec![20, 21], vec![7, 20, 21, 22]];
    let masks = [0x7FFFF, 0x3FFFFF, 0x7FFFFF];
    let mut regs = vec![0, 0, 0];
    for i in 0..64 { // Clock while inserting key bits
        for j in 0..regs.len() {
            regs[j] =
                clock(regs[j], &feedback_locs[j], &masks[j], read_pos(key, i));
        }
    }

    for i in 0..22 { // Clock while inserting frame bits
        for j in 0..regs.len() {
            regs[j] =
                clock(regs[j], &feedback_locs[j], &masks[j], read_pos(frame as u64, i));
        }
    }

    let clock_locs = [8, 10, 10];
    for _ in 0..100 { // Clock 100 times using majority rule
        let majority = majority(regs.clone()); // Could probably tweak majority() to remove clone
        for i in 0..3 {
            if read_pos(regs[i], clock_locs[i]) == majority {
                clock(regs[i], &feedback_locs[i], &masks[i], 0);
            }
        }
    }

    regs
}

fn clock(reg: u64, feedback_locs: &Vec<u8>, mask: &u64, input: u8) -> u64 {
    let mut next_bit = input;
    for loc in feedback_locs {
        next_bit ^= read_pos(reg, *loc);
    }
    ((reg << 1) | next_bit as u64) & *mask
}

pub(crate) fn generate_keystream(mut regs: Vec<u64>, length: usize) -> Vec<u8> {
    let feedback_locs = vec![vec![13, 16, 17, 18], vec![20, 21], vec![7, 20, 21, 22]];
    let masks = [0x7FFFF, 0x3FFFFF, 0x7FFFFF];
    let clock_locs = [8, 10, 10];
    let mut result = Vec::new();
    for _ in 0..length {
        let mut byte = 0u8;
        for j in 0..8 { // may need to reverse depending on little/big endian
            let majority = majority(regs.clone());
            for k in 0..3 {
                if read_pos(regs[k], clock_locs[k]) == majority {
                    regs[k] = clock(regs[k], &feedback_locs[k], &masks[k], 0);
                }
            }
            byte |=
                (read_pos(regs[0], 18)
                    ^ read_pos(regs[1], 21)
                    ^ read_pos(regs[2], 22)) << j;
        }
        result.push(byte);
    }
    result
}

// True if 1, False if 0
fn read_pos(num: u64, pos: u8) -> u8 {
    ((num >> pos) & 1) as u8
}

fn majority(regs: Vec<u64>) -> u8 {
    if read_pos(regs[0], 8)
        + read_pos(regs[1], 10)
        + read_pos(regs[2], 10) >= 2 {
        1
    } else {
        0
    }
}