use std::env;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    let num: u64 = args[1].parse().expect("Not a number.");

    for i in 0..num {
        
        println!("{} steps: {}", fibonacci(i), i);
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;
    
    println!("Time taken: {:?}", duration);
}

fn fibonacci(num: u64) -> u64 {

    if num <= 0 { 0 }

    else if num == 1 { 1 }

    else { fibonacci(num - 1) + fibonacci(num - 2) }
}
