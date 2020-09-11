extern crate rand;

// imported
use rand::Rng;

fn main() {
    let rand_nbr = rand::thread_rng().gen_range(1, 133);

    // random number from 1 to 132
    println!("Rand number {}", rand_nbr);

    // flip a coin
    // the expression `.gen_weighted_bool(probability)` is for example:
    // probability = 2
    // means that you have a chance that 1 in 2 of it being true, so P = 1/2
    let rand_bool = rand::thread_rng().gen_weighted_bool(2);

    println!("rand bool {}", rand_bool);
}
