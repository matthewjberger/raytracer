extern crate rand;

use rand::Rng;

pub fn drand48() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}
