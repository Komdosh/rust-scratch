pub(crate) fn ownership() {
    let v = vec![1, 2, 3];

    // let v2 = v;
    // println!("{:?}", v); // v invalidated after assign it to v2

    // let foo = |v:Vec<i32>| ();
    // foo(v);
    // println!("{:?}", v); // v invalidated after call foo(v);

    let u = 1;
    let _u2 = u; // i32 copy
    println!("u = {}", u);

    // let ub = Box::new(1);
    // let _u2 = ub;
    // println!("u = {}", *ub); //ub is invalid

    let print_vector = |x: Vec<i32>| -> Vec<i32>{
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{:?}", vv) // vv was returned in print_vector, same as v
}


