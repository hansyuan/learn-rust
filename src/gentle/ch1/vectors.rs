fn dump(arr: &[i32]) {
    println!("Array dump: {:?}", arr);
}

fn main(){
    // let vec = Vec::new(); // Compiler is helpful with this mistake
    
    let mut vec:std::vec::Vec<i32> = Vec :: new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    dump(& vec);

    for i in vec {
        println!("{}", i)
    }

    for i in 0..5 { 
        println!("{}", i);
    }

    for i in [0,1,2,3,4,5].iter() {
        println!("{}", i);
    }

    let bools = [true, false, true, true, false, true, false, false]; 

    for sliding_window in bools.windows(3) {
        println!("{:?}", sliding_window)
    }
}