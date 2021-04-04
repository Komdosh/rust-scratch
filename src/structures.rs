#![allow(dead_code)]

use core::{fmt, mem};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

struct Point<T: Display> {
    x: T,
    y: T,
}

impl<T: Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

fn origin() -> Point<f64> {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("Point({}, {})", p3.x, p3.y);
}

pub(crate) fn unions() {
    union IntOrFloat {
        i: i32,
        f: f32,
    }
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    fn process_value(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i: 42 } => {
                    println!("meaning of life value");
                }

                IntOrFloat { f } => {
                    println!("value = {}", f)
                }
            }
        }
    }

    process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { i: 5 });
}

fn use_slice(slice: &mut [i32]) {
    println!("first item = {}, len = {}", slice[0], slice.len());
    slice[0] = 128;
}

pub(crate) fn slices() {
    let mut data = [0, 1, 2, 3, 4];

    use_slice(&mut data[1..4]);
    println!("{:?}", data);
    use_slice(&mut data);
    println!("{:?}", data);
}

pub(crate) fn arrays() {
    let mut a: [i32; 5] = [0, 1, 2, 3, 4];

    println!("a has {} items, first is {}", a.len(), a[0]);
    a[0] = 128;
    println!("now a[0]={}", a[0]);

    println!("whole a={:?}", a);
    if a != [0, 1, 2, 3, 4] {
        println!("doesn't match");
    }
    if a == [128, 1, 2, 3, 4] {
        println!("rust array matching works")
    }

    let b = [1u16; 10];
    for i in 0..b.len() {
        print!("{}", b[i]);
    }
    println!();

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);
}

pub(crate) fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0}+{1} = {2}, {0}*{1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);
    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);

    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);
    let multiple_types = (true, 42.0, -1i8);
    println!("{:?}", multiple_types);

    let meaning_of_life = (42, );
    println!("{:?}", meaning_of_life)
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    return (x + y, x * y);
}

struct Line {
    start: Point<f64>,
    end: Point<f64>,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.end.x;
        let dy = self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub(crate) fn generics() {
    let a: Point<f64> = Point { x: 0.0, y: 16f64 };
    let b = Point { x: 1.2, y: 3.4 };

    let my_line = Line { start: a, end: b };
    println!("Line(({}); ({}))", my_line.start, my_line.end);
    println!("length = {}", my_line.len());
}


pub fn vectors() {
    let mut a = Vec::new();
    a.push(0);
    a.push(1);
    a.push(2);

    println!("a = {:?}", a);

    a.push(42);

    println!("a = {:?}", a);

    let idx: usize = 3;

    a[idx] = idx;
    println!("a[{}] = {}", idx, a[idx]);

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(3);
    println!("a = {:?}", a);

    let last_elem = a.pop();
    println!("last item is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

pub(crate) fn hash_map() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);

    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }

    println!("{:?}", shapes);
}

pub fn hash_set() {
    let mut greeks = HashSet::new();

    greeks.insert("gamma");
    greeks.insert("delta");

    println!("{:?}", greeks);
    greeks.insert("delta");

    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega!")
    }
    let added_vega_twice = greeks.insert("vega");
    if added_vega_twice {
        println!("vega was not added!")
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta");
    }
    println!("{:?}", greeks);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?}? - {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint
    println!("is {:?} a disjoint of {:?}? - {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    //union, intersection
    println!("items in either {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.union(&_6_10));
    println!("items in both {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.intersection(&_6_10));

    println!("difference of {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.difference(&_6_10));
    println!("symmetric difference of {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.symmetric_difference(&_6_10));
}

pub(crate) fn vector_objects() {
    trait Animal {
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
        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("{} says meow", self.name)
        }
    }

    impl Animal for Dog {
        fn name(&self) -> &'static str {
            self.name
        }
    }

    enum Creature {
        Dog(Dog),
        Cat(Cat),
    }

    let mut creatures = Vec::new();
    creatures.push(Creature::Dog(
        Dog { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { name: "Fluffy" }
    ));

    for c in creatures {
        match c {
            Creature::Dog(d) => d.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Dog{name: "John"}));
    animals.push(Box::new(Cat{name: "Fluffy"}));

    for a in animals.iter(){
        a.talk()
    }
}
