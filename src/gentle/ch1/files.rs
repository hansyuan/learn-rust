use std::env;
use std::fs::File;
use std::io::Read;
use std::io; 

fn read_to_string(filename: &str) -> Result<String, io::Error>{
    // Apparently supposed to be safer?

    // let mut file = match File::open(&filename) {
    //     Ok(f) => f, 
    //     Err(e) => return Err(e),
    // };

    // let mut text = String :: new();

    // match file.read_to_string(&mut text) {
    //     Ok(_) => Ok(text),
    //     Err(e) => Err(e),
    // }

    // Why is this better?
    let mut file = File :: open(& filename)?;
    let mut text = String :: new();
    file.read_to_string(& mut text)?;
    Ok(text)

    // Is this just syntactic sugar? 
}


fn main() {
    let filename = env :: args().nth(1).expect("Enter filename.");
    let text = read_to_string(&filename).expect("Bad file.");
    println!("File had {} bytes.", text.len())
}