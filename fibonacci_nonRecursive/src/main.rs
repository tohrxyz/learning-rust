use std::env;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: fibonacci <n>");
        return;
    }

    let num: u32 = args[1].parse().unwrap();
    for n in 0..num {
        println!("{}: \n {}", n, fibonacci(n));
    }
}
   
fn fibonacci(n: u32) -> BigUint {
    if n == 0 {
        return 0u32.to_biguint().unwrap();
    }

    let mut prev = 0u32.to_biguint().unwrap();
    let mut curr = 1u32.to_biguint().unwrap();

    for _ in 1..n {
        let next = prev.clone() + curr.clone();
        prev = curr.clone();
        curr = next;
    }

    curr
}
