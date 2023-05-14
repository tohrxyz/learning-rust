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

    // Now we can execute a simple SQL query. For example, to get the server version:
    let row = client.query_one("SELECT version()", &[]).await.unwrap();
    let version: &str = row.get(0);
    
    println!("Server version: {}", version);
}
