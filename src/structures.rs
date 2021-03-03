#![allow(dead_code)]

use core::mem;

struct Point<T> {
    x: T,
    y: T,
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

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub(crate) fn generics() {
    let a: Point<f64> = Point { x: 0.0, y: 16f64 };
    let b = Point { x: 1.2, y: 3.4 };

    let my_line = Line { start: a, end: b };
}
