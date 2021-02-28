pub(crate) fn scope_and_shadowing() {
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
