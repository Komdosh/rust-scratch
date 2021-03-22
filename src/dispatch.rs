
trait Printable{
    fn format(&self)->String;
}

impl Printable for i32{
    fn format(&self) -> String{
        format!("i32: {}", *self)
    }
}

impl Printable for String{
    fn format(&self) -> String{
        format!("string: {}", *self)
    }
}

fn print_it<T: Printable>(variable: T){ // compile time type
    println!("{}", variable.format())
} // monomorphisation

pub(crate) fn static_dispatch(){

    let integer = 123;
    let str = "hello".to_string();

    print_it(integer);
    print_it(str);
}
