pub(crate) fn functions() {
    print_value(32);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let first = 2;
    let second = 4;
    let prod = product(first, second);

    println!("{first} * {second} = {}", prod, first = first, second = second);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn print_value(x: i32) {
    println!("value = {}", x)
}

fn say_hello() { println!("hello"); }

pub(crate) fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };
    let val = 7;
    println!("{} + 1 = {}", val, plus_one(val));

    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };

        println!("{} + 2 = {}", 6, plus_two(2));
    }

    let borrow_two = &mut two;
    // T: by value
    // &T: by ref
    // &mut &: by ref mutable

    let plus_three = |x:&mut i32| *x += 3;
    let mut mut_val = 5;
    plus_three(&mut mut_val);
    println!("mut_val = {}", mut_val);
}
