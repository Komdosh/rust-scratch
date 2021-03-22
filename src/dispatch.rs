trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}


pub(crate) fn static_dispatch() {
    fn print_it<T: Printable>(variable: T) { // compile time type
        println!("{}", variable.format())
    } // monomorphisation

    let integer = 123;
    let str = "hello".to_string();

    print_it(integer);
    print_it(str);
}

pub(crate) fn dynamic_dispatch() {
    fn print_it_too(variable: &dyn Printable) {
        println!("{}", variable.format());
    }

    let integer = 123;
    let str = "hello".to_string();

    print_it_too(&integer);
    print_it_too(&str);
}


pub(crate) fn dispatch_in_action() {
    struct Circle { radius: f64 }

    struct Square { side: f64 }

    trait Shape {
        fn area(&self) -> f64;
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
    }

    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.0 },
        &Circle { radius: 2.0 },
        &Square { side: 4.0 },
    ];

    for(i, shape) in shapes.iter().enumerate(){
        println!("Shape #{} has area {}", i, shape.area())
    }
}

