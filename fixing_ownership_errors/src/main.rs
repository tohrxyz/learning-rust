fn main() {
    let string = return_a_string();
    println!("{}", string);
}

fn return_a_string() -> String {
  let s = String::from("Hello World");
  s
}
