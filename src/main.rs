trait Animal {
    fn make_sound(&self) -> &str;

    fn speak(&self) {
        println!("The animal says {}", self.make_sound());
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "woof"
    }

    fn speak(&self) {
        println!("The dog barks {}", self.make_sound());
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "meow"
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    dog.speak();
    cat.speak();
}
