use tokio_postgres::NoTls;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=subzero dbname=test_db", NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    loop {
        let name = input("Please enter name: ").unwrap();
        let email = input("Please enter email: ").unwrap();
    
        // Insert a new user
        client.execute(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&name, &email],
        ).await.unwrap();
    
        println!("Inserted new user");

        let add_another_user = input("Add another user? [y/n]: ").unwrap();

        if add_another_user == "y" {
            continue;
        } else {
            break;
        }
    }
}

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let len = line.trim_end_matches(&['\r', '\n'][..]).len();
    line.truncate(len);

    Ok(line)
}
