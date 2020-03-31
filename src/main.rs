use std::fs;
use std::io::{self, Write};
use rand::Rng;

mod utils;
mod tests;

#[allow(non_snake_case)]
fn main() {
    println!("---------------------------------------------------------");
    let mut mode = String::new();
    print!("Encrypt(0) or Decrypt(1): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    let mode:u8 = mode.trim().parse().expect("Please type a number!");
    if mode ==0 {
        let mut plain_text = String::new();
        print!("What is your plain text: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut plain_text).expect("Failed to read line");
        let plain_text = plain_text.trim();
        let block_size = rand::thread_rng().gen_range(1,plain_text.len());
        let output_OFB = utils::OFB(plain_text, block_size as u8);
    
        println!("Here is the cipher text '{}', written in file output.txt", output_OFB.2);
        fs::write("./output.txt", output_OFB.2).expect("Unable to write file!");
        println!("Please remember your key: {:?}, IV: {} and block size: {} ", output_OFB.0, output_OFB.1, block_size);
    }
    else if mode == 1 { 
        let mut path_output = String::new();
        let mut IV = String::new();
        let mut keys = String::new();
        let mut block_size = String::new();
        
        print!("What is your path to output file (example ./output.txt): " );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut path_output).expect("Failed to read line");
        let cipher_text = fs::read_to_string(path_output.trim()).expect("Unable to read file");

        print!("What is your keys (example 1,2,3): " );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut keys).expect("Failed to read line");
        let keys:Vec<u8> = keys.split(',').map(|n| n.trim().parse().expect("Failed to convert!")).collect();
        print!("What is your IV (example ahihe):" );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut IV).expect("Failed to read line");
        let IV = IV.trim();
        print!("What is your block size (example 10): " );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut block_size).expect("Failed to read line");
        let block_size: u8 = block_size.trim().parse().expect("Please type a number!");
        
        let dec_text = utils::decrypt_OFB(&cipher_text, block_size as u8, keys, &IV);
        println!("Answer: {}", dec_text);
    }
    else {
        println!("We don't have that. Sorry!");
    }
}
