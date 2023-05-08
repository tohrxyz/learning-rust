fn main() {
    struct Book {
        title: String,
        author: String,
        published_year: u32,
        num_pages: u32,
    }

    let book1 = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        published_year: 1938,
        num_pages: 336,
    };

    println!("Title: {}", book1.title);
    println!("Author: {}", book1.author);
    println!("Published in year: {}", book1.published_year);
    println!("Number of pages: {}", book1.num_pages);
    
    println!("");

    let mut book2 = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        published_year: 1938,
        num_pages: 336,
    };

    book2.title = String::from("Process");
    book2.author = String::from("Franz Kafka");
    book2.published_year = 1943;
    book2.num_pages = 295;

    println!("Title: {}", book2.title);
    println!("Author: {}", book2.author);
    println!("Published in year: {}", book2.published_year);
    println!("Number of pages: {}", book2.num_pages);
}
