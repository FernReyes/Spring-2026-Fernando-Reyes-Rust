use std::fs::File;
use std::io::{Write, BufReader, BufRead};

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {  //Saves books to "books.txt"
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro

    let mut bookfile = File::create(filename).unwrap();
    writeln!(bookfile, "{:?}", books).unwrap();
}

fn load_books(filename: &str) -> Vec<Book> {    //Loads books from "books.txt"
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new();

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}