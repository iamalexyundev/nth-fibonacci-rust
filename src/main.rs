use std::io;
fn main() {
    let mut index = String::new();
    println!("Enter n-th element of sequence:");
    let index: u32 = loop {
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
    let number = find_fib_recursive(index);
    // let number = find_fib_in_loop(index);
    println!("Fibonacci number is {number}");
}

fn find_fib_recursive(index: u32) -> u64 {
    match index {
        0 => 0,
        1 => 1,
        value => find_fib_recursive(value - 1) + find_fib_recursive(value - 2),
    }
    // if index == 0 || index == 1 {
    //     return u64::from(index);
    // } else {
    //     return find_fib_recursive(index - 1) + find_fib_recursive(index - 2);
    // }
}

// fn find_fib_in_loop(index: u32) -> u64 {
//     let mut count = 2;
//     let mut first: u64 = 0;
//     let mut second: u64 = 1;
//     let mut current: u64 = 0;

//     while count <= index {
//         current = first + second;
//         first = second;
//         second = current;
//         count += 1;
//     }
//     current
// }
