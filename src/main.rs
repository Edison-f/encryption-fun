mod rc4;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "rc4" => {
                rc4::rc4_runner()
            }
            _ => {
                println!("Cipher unknown/unsupported")
            }
        }
    } else {
        println!("No arguments supplied")
    }
    println!("\n\nFinished");
}
