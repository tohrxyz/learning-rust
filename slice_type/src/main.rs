use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_string = &args[1];

    let done_string = return_first_word(&start_string);

    println!("{}", done_string);
}

fn return_first_word(string: &String) -> String {
  let string = string.clone();
  let mut string_return = String::new();
  
  for i in 0..string.len() {
    let ch = string.chars().nth(i).unwrap();

    if ch != ' ' {
      string_return.push(ch);
    } else {
      break;
    }
  }

  string_return
}
