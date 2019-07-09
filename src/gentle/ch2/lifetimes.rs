// Copied from the example
#[derive(Debug)]
struct A {
    s: &'static str,
}

// Working from examples
#[derive(Debug)]
struct B<'b> {
    s: &'b str,
}

fn makes_b(s_param: &'static str) -> B<'static> {
    // let s = "waffles and chicken".to_string();
    B { s: &s_param }
}

fn choice(i: i32) -> &'static str {
    match i {
        0 => "zero",
        1 => "one",
        _ => "many",
    }
}

fn main() {
    let a = A { s: "hello dammit" };

    println!("{:?}", a);

    println!("{:?}", choice(1));

    let s = "literal".to_string();
    let b = B { s: &s };
    println!("{:?}", b);

    makes_b("wafflenator");
}
