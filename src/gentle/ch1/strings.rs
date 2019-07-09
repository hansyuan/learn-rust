fn dump (s: & str) {println!("\"{}\"", s)}

fn main() {
    let text = "hello world"; 
    let slice = text.to_string();

    dump(text); 
    dump(& slice);

    let mut s = String::new();
    s.push_str("mello ");
    s += "world";

    dump (& s); 

    assert!(s == "mello world");

    println!("Wait is this a lambda?"); 

    let f = | x | x + 1;

    println!("{:?}", f(1));

    let l = vec![1,2,6,2,4,6,2,3].into_iter();
    let results: Vec<i32> = l.map(|x| x*x).collect();

    println!("{:?}", results);

}