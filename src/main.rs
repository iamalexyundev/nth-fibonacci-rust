use std::io;
fn main() {
    let mut index = String::new();
    println!("Enter n-th element of sequence:");
    let index: i32 = loop {
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to get input");
        match index.trim().parse() {
            Ok(idx) => break idx,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    };
    let number = find_fib(index);
    println!("Fibonacci number is {number}");
}

fn find_fib(index: i32) -> i64 {
    if index == 0 {
        return 0;
    } else if index == 1 {
        return 1;
    } else {
        return find_fib(index - 1) + find_fib(index - 2);
    }
}
