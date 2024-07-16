enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

fn main() {
    let b1 = Book {
        isbn: 2,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 2,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 3,
        format: BookFormat::Hardback,
    };
    println!("{}", b1 == b2);
    println!("{}", b1 != b3);
    println!("{}", b2 != b3);
}
