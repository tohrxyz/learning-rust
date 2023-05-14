use tokio_postgres::NoTls;
use std::io::{self, Write};

#[tokio::main]
async fn main() {

    println!("[Logging into PostgreSQL server]");
    let username = input("Enter username: ").unwrap();
    let password = rpassword::read_password_from_tty(Some("Enter password: ")).unwrap();
    let db_name = input("Enter db name: ").unwrap();

    let login_credentials = format!("host=localhost user={} password={} dbname={}", &username, &password, &db_name);

    // Connect to the database
    let (client, connection) =
        tokio_postgres::connect(&login_credentials, NoTls)
        .await.unwrap()
    ;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("[Connected successfully to {}]", &db_name);

    loop {
        let name = input("Please enter name: ").unwrap();
        let email = input("Please enter email: ").unwrap();
    
        // Insert a new user
        client.execute(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&name, &email],
        ).await.unwrap();
    
        println!("[Inserted new user]");

        let add_another_user = input("Add another user? [y/n]: ").unwrap();

        match add_another_user.trim().to_lowercase().as_str() {
            "y" => {
                println!("[Continuing...]");
                continue;
            }
            _ => {
                println!("[Breaking...]");
                break;
            }
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
