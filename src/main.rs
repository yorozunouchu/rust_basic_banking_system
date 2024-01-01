trait Printable {
    fn print(&self);
}

fn print_something<T: Printable>(item: &T) {
    item.print();
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book: {}", self.title);
    }
}

struct Magazine {
    name: String,
}

impl Printable for Magazine {
    fn print(&self) {
        println!("Magazine: {}", self.name);
    }
}

fn main() {
    let book = Book {
        title: String::from("Rust Programming Language"),
    };
    let magazine = Magazine {
        name: String::from("The New Yorker"),
    };

    print_something(&book);
    print_something(&magazine);
}
