fn main() {
    println!("Hello World!");

    another_function(5, "beautiful".to_string());
}

fn another_function(x: i32, message: String) {
    println!("The value of x is {x} which is {message}.");
}