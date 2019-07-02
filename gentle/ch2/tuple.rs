// Minor tuples try out
fn add_points(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> (f64, f64) {
    (lat1+lat2, lon1+lon2)
}

fn main(){
    let new_point = add_points(49.5, -74.3, 2., -4.);
    println!("{:?}", new_point);
}