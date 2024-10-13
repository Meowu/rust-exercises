use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    year: u32,
    genre: BookGenre,
}

// impl Clone for Book {
//     fn clone(&self) -> Self {
//         Book {
//             title: self.title.clone(),
//             author: self.author.clone(),
//             year: self.year,
//             genre: self.genre.clone(),
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum BookGenre {
    Fiction,
    NonFiction,
    TextBook,
}

impl Book {
    fn new(title: &str, author: &str, year: u32, genre: BookGenre) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
            genre,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {} {} ({:?})",
            self.title, self.author, self.year, self.genre
        )
    }
}

#[derive(Debug)]
enum LibraryError {
    BookNotFound,
    DuplicateBook,
}

trait Library<T> {
    fn add_book(&mut self, book: T) -> Result<(), LibraryError>;
    fn remove_book(&mut self, title: &str) -> Result<T, LibraryError>;
    fn get_book(&self, title: &str) -> Option<&T>;
    fn books_by_genre(&self, genre: &BookGenre) -> Vec<&T>;
}

struct AdvancedLibrary<T> {
    books: HashMap<String, T>,
}

// 约束 AdvancedLibrary 的泛型参数 T 要求其实现 Clone
impl<T: Clone> AdvancedLibrary<T> {
    fn new() -> Self {
        AdvancedLibrary {
            books: HashMap::new(),
        }
    }
}

impl Library<Book> for AdvancedLibrary<Book> {
    fn add_book(&mut self, book: Book) -> Result<(), LibraryError> {
        if self.books.contains_key(&book.title) {
            Err(LibraryError::DuplicateBook)
        } else {
            self.books.insert(book.title.clone(), book);
            Ok(())
        }
    }

    fn remove_book(&mut self, title: &str) -> Result<Book, LibraryError> {
        self.books.remove(title).ok_or(LibraryError::BookNotFound)
    }

    fn get_book(&self, title: &str) -> Option<&Book> {
        self.books.get(title)
    }

    fn books_by_genre(&self, genre: &BookGenre) -> Vec<&Book> {
        self.books
            .values()
            .filter(|book| book.genre == *genre)
            .collect()
    }
}

fn main() {
    println!("Welcome to the library.");

    let mut library = AdvancedLibrary::new();

    let books = vec![
        Book::new(
            "The Rust Programming Language",
            "Steve Klabnik",
            2018,
            BookGenre::TextBook,
        ),
        Book::new("1984", "George Orwell", 1949, BookGenre::Fiction),
        Book::new(
            "To Kill a Mockingbird",
            "Harper Lee",
            1960,
            BookGenre::Fiction,
        ),
        Book::new(
            "A Brief History of Time",
            "Stephen Hawking",
            1988,
            BookGenre::NonFiction,
        ),
    ];

    for book in books {
        match library.add_book(book) {
            Ok(_) => println!("Added book successfully"),
            Err(LibraryError::DuplicateBook) => println!("Book already exists"),
            Err(_) => println!("Unknown error occurred"),
        }
    }

    match library.add_book(Book::new("1984", "George Orwell", 1949, BookGenre::Fiction)) {
        Ok(_) => println!("Added book successfully"),
        Err(LibraryError::DuplicateBook) => println!("Book already exists"),
        Err(_) => println!("Unknown error occurred"),
    }

    if let Some(book) = library.get_book("1984") {
        println!("Found book: {}", book);
    }

    let fiction_books = library.books_by_genre(&BookGenre::Fiction);
    println!("Fiction books:");
    for book in fiction_books {
        println!(" {}", book);
    }

    match library.remove_book("A Brief History of Time") {
        Ok(book) => println!("Removed book: {}", book),
        Err(LibraryError::BookNotFound) => println!("Book not found"),
        Err(_) => println!("Unknown error occurred"),
    }
}
