#![allow(dead_code)]

use core::mem;

mod sh;

const MEANING_OF_LIFE: u8 = 42; //no fixed address

static mut Z: i32 = 64;

fn main() {
    // data_types_playground()
    // scope_and_shadowing();
    // constants();
    // if_statement()
    // sh::stack_and_heap();
    // while_loop();
    // for_loop();
    country_matcher()
}

fn country_matcher(){
    let country_code = 46;

    let country = match country_code {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country)
}

fn while_loop() {
    let mut i = 1;
    while i < 1000 {
        i = i << 1;
        println!("i = {}", i);
    }

    println!("now unconditional loop");
    i = 1;
    loop {
        i <<= 1;
        println!("i = {}", i);
        if i > 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 { // [1, 11)
        println!("x = {}", x);
    }

    for x in 1..=5 { // [1, 5]
        println!("x = {}", x);
    }

    println!("now pos and number enumerate");
    for(pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y)
    }
}

fn constants() {
    println!("constant meaning of life is {}", MEANING_OF_LIFE);
    unsafe {
        println!("static {}", Z);
        Z = 128;
        println!("static changed {}", Z);
    }
}

fn if_statement() {
    let a = 0;
    if i32::pow(a, 0) == 1 {
        println!("it is true");
    } else if a == 5 {
        println!("shouldn't be called");
    }
    let some_value = if a == 0 { "ok" } else { "not ok" };
    println!("how is going? - {}", some_value);
}

fn scope_and_shadowing() {
    let a = 0;

    println!("initial a = {}", a);

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
