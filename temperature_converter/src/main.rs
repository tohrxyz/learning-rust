use std::io;

fn main(){

    let mut input: String = String::new();
    loop {
        println!("Enter Fahrenheit (F) or Celsius (C)? :");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        
        if input.trim() != "F" && input.trim() != "C" {
            continue;
        }
        else {
            input = input.trim().to_string();
            break;
        }
    }

    let mut value: String = String::new();
    let mut degree_value: f64;

    loop {
        println!("Enter value for {input}: ");

        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value.");

        degree_value = match value
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => continue, 
            };
        break;
    }

    let result = convert_f_c(&input, degree_value);

    println!("Result: {}{} = {:.2}{}", degree_value, &input, result, check_which_unit(&input));

    
}

fn convert_f_c(unit: &str, value: f64) -> f64 {
    if unit == "F" {
        return (value - 32.0) * 5.0/9.0;
    } 
    if unit == "C" {
        return (value * (9.0/5.0)) + 32.0;
    }
    else {
        return 0.0;
    }
}

fn check_which_unit(unit: &str) -> String{
    if unit == "F" {
        return "C".to_string();
    } else {
        return "F".to_string();
    }
}