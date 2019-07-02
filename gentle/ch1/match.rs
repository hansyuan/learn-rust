use std::env::args;

fn main() {
    let n: Vec<String> = args().skip(1).collect();
    let nn = n.get(0);

    if ! nn.is_some() {
        return
    }

    let nn: &str = nn.unwrap();
    let nnn: i32 = nn.parse::<i32>().unwrap();

    let result: &str = match nnn {
        0 => "Hello",
        1 => "World",
        _ => "WHAT"
    };

    println!("{}", result);
    
}
