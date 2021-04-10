extern crate rand;
extern crate phrases;

use rand::Rng;
use phrases::greetings;
use phrases::greetings::french;

pub(crate) fn external_crate() {
    let mut rng = rand::thread_rng();
    let _b: bool = rng.gen();
}

pub(crate) fn own_lib() {
    println!("English: {}, {}", greetings::english::hello(),
             greetings::english::goodbye());

    println!("French: {}, {}", french::hello(),
             french::goodbye());
}
