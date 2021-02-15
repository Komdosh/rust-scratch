use core::mem;

fn main() {
    data_types_playground()
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
