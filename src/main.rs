fn main() {
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    shapes.push(Box::new(Circle { radius: 5.0 }));
    shapes.push(Box::new(Rectangle { width: 4.0, height: 6.0}));

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}
