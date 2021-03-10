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
