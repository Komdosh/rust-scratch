
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

fn print_it_too(variable: &dyn Printable){
    println!("{}", variable.format());
}

pub(crate) fn static_dispatch(){

    let integer = 123;
    let str = "hello".to_string();

    print_it(integer);
    print_it(str);
}

pub(crate) fn dynamic_dispatch(){

    let integer = 123;
    let str = "hello".to_string();

    print_it_too(&integer);
    print_it_too(&str);
}
