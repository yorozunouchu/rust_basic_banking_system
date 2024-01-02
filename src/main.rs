struct Person<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Peter");
    let person = Person { name: &name };
    println!("Hello, {}!", person.name);
}
