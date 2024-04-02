use std::env;

mod rc4;
mod tea;
mod a51;

// Todo: Add ECB, CBC, CTR modes if applicable

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].to_lowercase().as_str() {
            "rc4" => {
                rc4::rc4_runner()
            }
            "tea" => {
                tea::tea_runner()
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
