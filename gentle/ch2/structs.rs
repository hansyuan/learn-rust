
#[derive(Debug)] // Allows debug outputs like a dictionary
struct Spacetime {
    x: i64,
    y: i64,
    z: i64,
    t: i64, 
}

impl Spacetime {
    
    // A constructor, probably. 
    // Is this a static method?
    fn new(x:i64, y:i64, z:i64, t:i64) -> Spacetime {
        Spacetime {
            x: x,
            y: y,
            z: z, 
            t: t
        }
    }

    fn coordinates(&self) -> String {
        format!("(x, y, z, t): ({}, {}, {}, {})", 
        self.x, self.y, self.z, self.t)
    }
}

fn main () {

    // Struct way of creating 
    // let snapshot = Spacetime{
    //     x: 3, y: 2, z: 5, t:4,
    // };

    // Constructor way of creating
    let snapshot = Spacetime :: new(1, 2, 3, 4);

    println!("spacetime x {} t {}", snapshot.x, snapshot.t);
    println!("spacetime z {} y {}", snapshot.z, snapshot.y);

    println!("spacetime: {}", snapshot.coordinates()); 

    println!("{:?}", snapshot); // This is possible because of the derive debug 
}