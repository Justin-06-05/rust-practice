use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    random_vector();
}

fn random_vector() {
    let mut vec: Vec<u32> = (0..10).collect();
    vec.shuffle(&mut thread_rng());
    println!("{:?}", vec);
}