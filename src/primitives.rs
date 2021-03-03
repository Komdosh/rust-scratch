use core::mem;

const MEANING_OF_LIFE: u8 = 42; //no fixed address

static mut Z: i32 = 64;

pub fn data_types_playground() {
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

pub fn constants() {
    println!("constant meaning of life is {}", MEANING_OF_LIFE);
    unsafe {
        println!("static {}", Z);
        Z = 128;
        println!("static changed {}", Z);
    }
}

pub(crate) fn enums() {
    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8, u8, u8),
        CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
    }

    let c: Color = Color::CmykColor { cyan: 211, magenta: 224, yellow: 12, black: 255 };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255 }
        | Color::CmykColor { black: 255, .. } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => ()
    }
}
