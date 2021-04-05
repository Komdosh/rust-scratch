use std::rc::Rc;
use std::sync::Arc;
use std::thread;

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


pub(crate) fn borrowing() {
    let v = vec![1, 2, 3];


    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
        // x.push(4);
    };

    print_vector(&v); // borrow v for a while
    println!("{:?}", v);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);

    let z = vec![3, 2, 1];

    for i in &z {
        println!("i = {}", i);
    }
}

pub(crate) fn lifetime() {
    #[derive(Debug)]
    struct Person {
        name: String
    }

    impl Person {
        fn get_ref_name(&self) -> &String {
            &self.name
        }
    }

    #[derive(Debug)]
    struct Company<'lifetime_company> {
        name: String,
        ceo: &'lifetime_company Person,
    }

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    println!("{:?}", tesla)
}

pub(crate) fn lifetime_in_struct() {
    #[derive(Debug)]
    struct Person<'lifetime_person> {
        name: &'lifetime_person str
    }

    impl<'lifetime_person_impl> Person<'lifetime_person_impl> {
        fn talk(&self) {
            println!("Hi, my name is {}.", self.name);
        }
    }

    let boss = Person { name: "Elon Musk" };
    boss.talk();
}


pub(crate) fn ref_count_demo() {
    struct Person {
        name: Rc<String>
    }

    impl Person {
        fn new(name: Rc<String>) -> Person {
            Person { name }
        }
        fn greet(&self) {
            println!("Hi, my name is {}.", self.name);
        }
    }

    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    println!("Name = {}", name);
}

pub(crate) fn atomic_ref_count_demo() {
    struct Person {
        name: Arc<String>
    }

    impl Person {
        fn new(name: Arc<String>) -> Person {
            Person { name }
        }
        fn greet(&self) {
            println!("Hi, my name is {}.", self.name);
        }
    }
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());
    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}", name);
    t.join().unwrap()
}
