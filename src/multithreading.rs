use std::sync::{Mutex, Arc};
use std::thread;

pub(crate) fn mutex() {
    struct Person {
        name: Arc<String>,
        state: Arc<Mutex<String>>
    }

    impl Person {
        fn new(name: Arc<String>,state: Arc<Mutex<String>>) -> Person {
            Person { name, state }
        }
        fn greet(&self) {
            let mut state = self.state.lock().unwrap();

            state.clear();
            state.push_str("excited");

            println!("Hi, my name is {} and I am {}.", self.name, state);

        }
    }
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        person.greet();
    });

    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap()
}
