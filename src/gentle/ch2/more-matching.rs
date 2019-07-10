// matching and dewrapping

fn main() {
    let a = (1, 2, 3);
    let (x, y, z) = a;

    println!("{:?}", a);

    let a = ("1".to_string(), "2".to_string(), "3".to_string());
    let (ref x, ref y, ref z) = a;

    println!("{:?}", a);

    // deconstructing works with structs too
    // (cool)
    let t = Tup {
        b: 1,
        c: "2".to_string(),
        a: 3,
    };
    let Tup { a, b, c } = t;

    let d = c;

    println!("{:?}", (a, b, d));

    match_tup(t);
}

struct Tup {
    a: i32,
    b: i32,
    c: String,
}

fn match_tup(t: Tup) {
    let text = match t {
        Tup { a: 3, b: 1, c: s } => format!("first {}", s),
        Tup {
            a: 3,
            b: 1,
            c: ref s,
        } => format!("second {}", s),
        _ => format!("No match"),
    };
    println!("{}", text);
}
