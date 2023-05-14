use tokio_postgres::NoTls;

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

    // Create table
    client.execute(
        "CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE
        )",
        &[],
    ).await.unwrap();

    println!("Table users created");
}
