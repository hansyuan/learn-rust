fn main() {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    let mut vec3 = Vec::new();

    vec1.push("1".to_string());
    vec2.push("2".to_string());
    vec3.push("3".to_string());

    for n in &vec1 {
        println!("{:?}", n);
    }

    for n in &mut vec2 {
        println!("{:?}", n);
    }

    // for n in vec3 {
    //     println!("{:?}", n)
    // }

    println!("{:?}", vec1);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
    // vec3 can only be printed if the above for loop did not consume it

    println!("{:?}", vec1);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
    // Just checking.

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    for n in v.iter().map(|n| n * n) {
        println!("{:?}", &n);
    }
}
