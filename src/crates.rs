extern crate rand;

use rand::Rng;

pub(crate) fn external_crate() {
    let mut rng = rand::thread_rng();
    let _b: bool = rng.gen();
}
