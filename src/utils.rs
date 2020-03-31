pub const UPPERCASE_START: u8 = 65;
pub const UPPERCASE_END: u8 = 65 + 25;
pub const LOWERCASE_START: u8 = 97;
pub const LOWERCASE_END: u8 = 97 + 25;
pub const ALPHABET_SIZE: u8 = 26;
pub const ALL_CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn shift_alphabet(mut num: u8, is_lower: bool) -> u8 {
    if is_lower {
        if num < LOWERCASE_START {
            num += ALPHABET_SIZE;
        } else if num > LOWERCASE_END {
            num -= ALPHABET_SIZE;
        }
    } else {
        if num < UPPERCASE_START {
            num += ALPHABET_SIZE;
        } else if num > UPPERCASE_END {
            num -= ALPHABET_SIZE;
        }
    }    num
}


pub fn shift_cipher(plain_text: &str, k: u8, is_encrypt: bool) -> String {
    let mut cipher_text = String::new();
    for c in plain_text.chars() {
        if c.is_ascii_whitespace() {
            cipher_text.push(c);
        }
        let mut num = (c as u8) - (k%ALPHABET_SIZE);
        if is_encrypt == false {
            num = (c as u8) + (k%ALPHABET_SIZE);
        }
        let new_c: char = shift_alphabet(num, c.is_ascii_lowercase()) as char;
        cipher_text.push(new_c);
    }
    cipher_text
}

pub fn padding(plain_text: &str, default_num_pad: u8) -> String{
    let mut new_plain_text = String::from(plain_text);
    let mut start: u8 = 65;
    loop {
        new_plain_text.push(start as char);
        start += 1;
        if new_plain_text.len() % (default_num_pad as usize) == 0{
            break;
        }
    }
    new_plain_text
}

pub fn remove_padding(text_padded: &str) -> String {
    let mut removed_pad_text = String::from(text_padded);
    let mut last_c = text_padded.chars().last().unwrap() as u8;
    loop {
        removed_pad_text.pop();
        last_c -=1;
        if last_c == 64 {
            break;
        }
    }
    removed_pad_text
}

use rand::Rng;
#[allow(non_snake_case)]
pub fn generate_IV(size_block: u8)->String {
    let mut IV = String::new();
    for _i in 0..size_block{
        let idx = rand::thread_rng().gen_range(0, ALL_CHARACTERS.len());
        IV.push(ALL_CHARACTERS.chars().nth(idx).unwrap());
    }
    IV 
}

#[allow(non_snake_case)]
pub fn a_xor_b(text_a: &str, text_b: &str) -> String{
    // a b have to have the same length
    let mut res = String::new();
    let u8a = text_a.as_bytes();
    let u8b = text_b.as_bytes();
    for i in 0..u8a.len() {
        res.push( (u8a[i] ^ u8b[i] ) as char);
    }
    res
}


#[allow(non_snake_case)]
pub fn OFB(plain_text: &str, block_size: u8) -> (Vec<u8>, String, String){
    let mut keys = Vec::new();
    let IV = generate_IV(block_size);
    let mut cipher_text = String::new();

    
    let plain_text_padded = padding(plain_text, block_size);
    let size_text = plain_text_padded.len();
    let num_blocks = size_text / (block_size as usize); 
    let mut output_enc_text_prev = IV.clone();
    for i in 0..num_blocks{
        let k = rand::thread_rng().gen_range(0,26);
        keys.push(k);
        let output_enc_i = shift_cipher(&output_enc_text_prev, k, true);
        output_enc_text_prev = output_enc_i.clone();
        cipher_text.push_str(&a_xor_b(&plain_text_padded[(i*block_size as usize)..(i*block_size as usize+block_size as usize)], &output_enc_i));
    }
    (keys, IV, cipher_text)
}

#[allow(non_snake_case)]
pub fn decrypt_OFB(cipher_text: &str, block_size: u8, keys: Vec<u8>, IV: &str) -> String {
    let mut plain_text = String::new();

    let size_text = cipher_text.len();
    let num_blocks = size_text / (block_size as usize); 
    let mut output_enc_text_prev = String::from(IV.clone());
    for i in 0..num_blocks{
        let k = keys[i];
        let output_enc_i = shift_cipher(&output_enc_text_prev, k, true);
        output_enc_text_prev = output_enc_i.clone();
        plain_text.push_str(&a_xor_b(&cipher_text[(i*block_size as usize)..(i*block_size as usize+block_size as usize)], &output_enc_i));
    }
    remove_padding(&plain_text)
}