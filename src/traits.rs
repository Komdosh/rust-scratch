use std::fmt::Debug;

pub fn traits() {
    trait Animal {
        fn create(name: &'static str) -> Self;
        fn name(&self) -> &'static str;

        fn talk(&self) {
            println!("{} cannot talk", self.name());
        }
    }

    struct Cat {
        name: &'static str
    }

    struct Dog {
        name: &'static str
    }

    impl Animal for Cat {
        fn create(name: &'static str) -> Cat {
            Cat { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("{} says meow", self.name)
        }
    }

    impl Animal for Dog {
        fn create(name: &'static str) -> Dog {
            Dog { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }

    let cat = Cat::create("Mr. Whiskers");
    cat.talk();
    let dog: Dog = Animal::create("Mint Butler");
    dog.talk();


    trait Summable<T> {
        fn sum(&self) -> T;
    }

    impl Summable<i32> for Vec<i32> {
        fn sum(&self) -> i32 {
            let mut result: i32 = 0;
            for x in self {
                result += *x;
            }
            return result;
        }
    }

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum())
}

pub(crate) fn trait_parameters() {
    #[derive(Debug)]
    struct Circle {
        radius: f64
    }
    #[derive(Debug)]
    struct Square {
        side: f64
    }

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

    // fn print_info(shape: impl Shape + Debug) {
    // fn print_info<T: Shape+Debug>(shape: T) {
    fn print_info<T>(shape: T)
        where T: Shape + Debug
    {
        println!("{:?}", shape);
        println!("The area is {}", shape.area())
    }

    let circle = Circle { radius: 2.0 };
    print_info(circle);
}

pub(crate) fn trait_into() {
    struct Person {
        name: String
    }

    impl Person {
        /*        fn new(name: &str) -> Person{
                    Person {name: name.to_string()}
                }*/

        fn new<S: Into<String>>(name: S) -> Person {
            Person { name: name.into() }
        }
    }

    let john = Person::new("John");

    let name: String = "Jane".to_string();
    let jane = Person::new(name/*.as_ref()*/);
}

pub(crate) fn trait_drop() {
    struct Creature {
        name: String
    }

    impl Creature {
        fn new(name: &str) -> Creature {
            println!("{} enters the game", name);
            Creature { name: name.into() }
        }
    }

    impl Drop for Creature {
        fn drop(&mut self) {
            println!("{} is dead", self.name)
        }
    }

    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        drop(goblin);
        println!("end of scope")
    }
}
