enum Human {
    Yes,
    No
}

fn main() {
    let person = Human::Yes;
    let dog = Human::No;

    match person {
        Human::Yes => {
            println!("person is human");
        },
        Human::No => {
            println!("person is not human");
        }
    }
    match dog {
        Human::Yes => {
            println!("dog is human");
        },
        Human::No => {
            println!("dog is not human");
        }
    }
}
