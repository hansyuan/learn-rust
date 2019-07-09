// Function syntax

fn sum(fst: f64, snd: f64) -> f64 {
    fst + snd
}

fn fac(n: u128) -> u128 {
    let result = if n == 0 {1} else {n * fac(n - 1)};
    println!("{}", result); 
    result
}

fn rec(n: u128) {
    println!("{}", n);
    if n == 0 {return}
    rec(n-1);
}

fn main() {
    // let a = sum(2.0, 2.0); 
    // println! ("{}", a);

    // let n = 40;
    // fac(n);

    // rec(1000000000); // okay, likely not tail recursive. 

    let n: i32 = 42; 
    let n_ref = & n;
    println!("read pointer: {}. read pointer explicitly: {:p}", n_ref, n_ref)


}

