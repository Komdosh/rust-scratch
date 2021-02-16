use core::mem;

const MEANING_OF_LIFE: u8 = 42; //no fixed address

static mut Z: i32 = 64;

fn main() {
    // data_types_playground()
    // scope_and_shadowing();

    constants()
}

fn constants() {
    println!("constant meaning of life is {}", MEANING_OF_LIFE);
    unsafe {
        println!("static {}", Z);
        Z = 128;
        println!("static changed {}", Z);
    }
}

#[allow(dead_code)]
fn scope_and_shadowing() {
    let a = 0;

    if i32::pow(a, 0) == 1 {
        println!("it is true");
    }

    let a = 2; //overriding

    {
        println!("inner scope, outside a = {}", a);

        let a = 1;

        println!("inner scope, new inside a = {}", a);

        let b = 16;
        if b == 16 {
            println!("inner scope, new inside b = {}", b)
        }
    }
    println!("outside a = {}", a);
}

#[allow(dead_code)]
fn data_types_playground() {
    let first: u8 = 127;
    let mut second: u8 = 128;

    println!("first ({}) + second ({}) = sum({})", first, second, first + second);

    second = 16;
    println!("second changed! first ({}) + second ({}) = sum({})", first, second, first + second);

    let large_num = 123;

    println!("large_num ({}), size ({}) bytes", large_num, mem::size_of_val(&large_num));

    let raw: isize = 128;
    let size_of_raw = mem::size_of_val(&raw);
    println!("raw ({}), takes up ({}) bytes, {}-bit OS", raw, size_of_raw, size_of_raw * 8);

    let char = 'x';

    println!("char ({}), size ({}) bytes", char, mem::size_of_val(&char));

    let double = 2.5;
    println!("double ({}), size ({}) bytes", double, mem::size_of_val(&double));

    let bool = true;
    println!("bool ({}), size ({}) bytes", bool, mem::size_of_val(&bool));
}
