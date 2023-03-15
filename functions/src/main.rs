fn main() {
    println!("Hello World!");

    another_function(5, "beautiful".to_string());

    let x = five();
    println!("Amazing score: {x}");

    let y = plus_one(21);
    println!("Plus one: {y}");
}

fn another_function(x: i32, message: String) {
    println!("The value of x is {x} which is {message}.");
}

fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 {
    x + 1
}