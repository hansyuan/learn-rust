// Divide by float

trait Divide {
    fn div(&self) -> f64; 
}

impl Divide for i32 {
    fn div(&self) -> f64 {
        *self as f64 / 2.0
    }
}

impl Divide for f64 {
    fn div(&self) -> f64 {
        *self as f64 / 4.0
    }
}

fn main() {
    let a = 42; 
    let b = 3.24; 

    println!("{}", a.div());
    println!("{}", b.div());
}