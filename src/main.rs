use std::env;

mod rc4;
mod tea;
mod a51;
mod substitution;

/** Todo:   Add ECB, CTR, CBC modes if applicable
            Add util file for common actions like XORing keystream or getting input
 */
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
            "a5/1" => {
                a51::a51_runner()
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
