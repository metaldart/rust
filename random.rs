use std::rand::{task_rng, Rng};

fn main() {
    // a number from [-100, 100)
    let num: f64 = task_rng().gen_range(-100, 100);
    println!("{}", num);
}