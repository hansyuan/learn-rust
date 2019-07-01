
fn product(vals: &[i32]) -> i32 {
    
    let mut prod = 1;
    
    for index in 0..vals.len() {
        prod *= vals[index]; 
    }

    prod 
}

fn main () { 
    let a = [1,2,3,4]; 

    // Apparently a useful trick to finding the type of an expr is
    // to declare its var as a none type, ().
    
    // let a: () = true;

    println!("1st: {}", a[0]);
    println!("len: {}", a.len());

    
    for i in 0..a.len() {
        println!("{}", i)
    }

    let p = product(&a);
    // Apparently this means that product() 'borrowed' the a array.
    println!("{}", p);

    println!("{:?} , {:p}", &a, &a);
    // This is pretty cool.

    // assert!(fail == ()) // Okay, this is not what I thought it'd be.

    println!("{:?}", &a[0..2]);

    let fail = a.get(5);
    println!("fail contains {:?}", fail);
    println!("fail is something? {}", fail.is_some());

    let last = fail.unwrap_or(&-9999);
    println!("{}", last);
    

}