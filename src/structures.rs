#![allow(dead_code)]

use core::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin()->Point{
    Point{x: 0.0, y: 0.0}
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

fn use_slice(slice: &mut[i32]) {
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
