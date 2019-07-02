// hello world 

/* This should also be a comment */ 

fn main() {
    /* I'm guessing a function header would go here */ 

    assert_eq!(false, false);


    let ans = 42; 
    println!("String {}", ans);
    
    for i in 0..21{
        if i % 2 == 0 {
            println!("Even: [{}]", i)
        }
        else {
            println!("Odd: [{}]", i)
        }
    }

    for i in 0..5 {
        let mystery = if i % 2 == 0 {"even"} else {"odd"}; 
        println!("That was almost pythonic: {}", mystery); 
    }

    let mut variable = 0.0;
    for i in 0..10 {
        variable += i as f64; 
    }
    println!("{}", variable)
}