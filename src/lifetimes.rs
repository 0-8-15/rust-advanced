struct Book<'a> {
    title: &'a str,
    cover_text: String,
}

trait Cover {
    fn get_title(&self) -> &str;
    fn get_cover_text(&self) -> &str;
}

impl <'a>Cover for Book<'a>{
    fn get_title(&self) -> &str {
        self.title
    }

    fn get_cover_text(&self) -> &str {
        &self.cover_text
    }
}

struct Inventory<'a> {
    books: Vec<& 'a Book<'a>>,
}

impl <'a>Inventory<'a> {
    fn add_book(&mut self, book: &Book) {
    }

    fn show_titles(&self) {
        for book in &self.books {
            println!("Title: {}", book.get_title());
        }
    }
}
fn main() {
    let book1 = Book {
        title: "The Lord of the Rings",
        cover_text: String::from("The Fellowship of the Ring"),
    };

    let book2 = Book {
        title: "The Hitchhiker's Guide to the Galaxy",
        cover_text: String::from("Don't Panic"),
    };

    let book3 = Book {
        title: "Dr. Who",
        cover_text: String::from("The Doctor"),
    };

    let favorite_book_id: &usize; // Nicht anpassen
    {
        // Diesen Block nicht löschen. (Aber gern den Inhalt anpassen)
        let book_id: usize = 1;
        favorite_book_id = &book_id;
    }

    let mut my_inventory = Inventory { books: Vec::new() };
    my_inventory.add_book(&book1);
    my_inventory.add_book(&book2);
    my_inventory.add_book(&book3);

    my_inventory.show_titles();

    println!("{}", my_inventory.books[0].get_cover_text())
}
