use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    if rng.r#gen() {
        println!("True was randomly generated.");
    } else {
        println!("False was randomly generated.");
    }
    let x: f64 = rng.r#gen();
    let y = rng.gen_range(-10.0, 10.0);
    println!("x is: {}", x);
    println!("y is: {}", y);
    println!("Random Number between 0 and 9: {}", rng.gen_range(0, 10));
}