use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let start_string = &args[1];
  let done_string = return_first_word(&start_string);

  println!("First word: {}", done_string);
}

fn return_first_word(string: &String) -> &str {
  let bytes = string.as_bytes();

  for(i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &string[0..i];
    }
  }

  &string[..]
}