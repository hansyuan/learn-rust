// enum in full glory

#[derive(Debug)]
enum Point {
    Latitude(f64),
    Longitude(f64),
    Name(String),
}

// implement a dumper to see move logic vs copy logic
fn dump(pt: &Point) {
    use Point::*;

    match *pt {
        Latitude(l) => println!("Latitude: {}", l),
        Longitude(l) => println!("Longitude: {}", l),
        Name(ref n) => println!("Name: {}", n),
    }
}

fn main() {
    let lat = Point::Latitude(49.65);
    let lon = Point::Longitude(-96.05);
    let on_earth = Point::Name("On Earth".to_string());

    println!("{:?} {:?} {:?} ", lat, lon, on_earth);
    println!("{:?} ", Point::Latitude(57.34));

    dump(&lat);
    dump(&lon);
    dump(&on_earth);
}
