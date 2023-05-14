use tokio_postgres::NoTls;
use std::io::{self, Write};

struct Login {
    username: String,
    password: String,
    db_name: String,
}

#[tokio::main]
async fn main() {

    println!("[Logging into PostgreSQL server]");
    
    let mut login = Login {
        username: String::new(),
        password: String::new(),
        db_name: String::new(),
    };
    
    loop {
        login.username = input("Enter username: ").unwrap();
        login.password = rpassword::read_password_from_tty(Some("Enter password: ")).unwrap();
        login.db_name = input("Enter db name: ").unwrap();
    
        let login_credentials = format!("host=localhost user={} password={} dbname={}", &login.username, &login.password, &login.db_name);
    
        let connection_result = tokio_postgres::connect(&login_credentials, NoTls).await;
        
        match connection_result {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                
                println!("[Connected successfully to {}]", &login.db_name);

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
                break;
            }
            Err(_) => {
                eprintln!("[There was a problem with the connection]");
                continue;
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
