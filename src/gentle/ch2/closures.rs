// aka lambdas

fn main() {
    let f = |x: i32| println!("{:?}", x);
    f(25);

    let g = |y| 2.0 * y;

    println!("{:?}", g(1.5)); // ok so this will lock in the param type

    // println!("{:?}", g(5));

    let g = 9.8;

    // gravity and change of height
    // h - gt^2
    let dh = |t: f32, h: f32| h - (g * (t.powf(2.0)));

    println!("{:?}", dh(3., 200.));
    // println!("{:?}, ")
}
