pub(crate) fn while_loop() {
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

pub(crate) fn for_loop() {
    for x in 1..11 { // [1, 11)
        println!("x = {}", x);
    }

    for x in 1..=5 { // [1, 5]
        println!("x = {}", x);
    }

    println!("now pos and number enumerate");
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y)
    }
}

