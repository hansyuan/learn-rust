// implementing enums

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn as_str(&self) -> &'static str {
        match *self {
            Dir::Up => "^",
            Dir::Down => "v",
            Dir::Left => "<",
            Dir::Right => ">",
        }
    }
}

impl Dir {
    fn next(&self) -> Dir {
        use Dir::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Speed {
    Slow = 1,
    Medium,
    Fast,
}

impl Speed {
    fn next(&self) -> Speed {
        use Speed::*;
        match *self {
            Slow => Medium,
            Medium => Fast,
            Fast => Slow,
        }
    }
}

fn main() {
    println!("Main. ");
    println!("{:?}", Dir::Up.as_str());

    let mut d = Dir::Up;

    for _ in 0..8 {
        println!("d {:?}", d);
        println!("{}", d == Dir::Up);
        d = d.next();
    }

    let mut s = Speed::Slow;
    for _ in 0..6 {
        println!("s {:?}", s);
        println!("{:?} <  {:?}: {}", s, Speed::Fast, s < Speed::Fast);
        println!("{:?} == {:?}: {}", s, Speed::Fast, s == Speed::Fast);
        s = s.next();
    }
}
