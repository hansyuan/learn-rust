use std :: env :: args;

fn main() {
    let cli_args: Vec<String> = args().collect();
    for arg in cli_args {
        println!("{}", arg)
    }
}