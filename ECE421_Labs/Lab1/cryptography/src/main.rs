// Q1: This algorithm is trying to find a candidate prime integer, by checking the primality of the first 64 bits of that integer.
// Presumably, if that integer is not prime, we can discard the candidate. If it is prime, we need to investigate further.

// Q2: Here the parameter is the bitwidth (i.e. for an integer with max value of 2^32, 32)
// use apint::*;
// use is_prime::*;
// fn function(width: BitWidth) -> ApInt {
//     loop {
//         let mut candidate = ApInt::random_with_width(width);
//         candidate.set_bit_at(0);
//         let i = candidate.as_string_with_radix(10);
//         if is_prime(&i)== true {
//             return candidate;
//         }
//     }
// }

// Q3: we rewrite the candidate int in the form d*2^s+1. We then have four iterations...

// Q4: glass_pumpkin is the rust crate.

// Q5:
use primes::{Sieve, PrimeSet};
fn main() {
    let mut x_max = 1;
    let mut y = 0;
    let mut first_prime = 0;

    let mut pset = Sieve::new();

    let primes_under_1000: Vec<u64> = pset.iter().take_while(|&p| p < 1000).collect();
   
    for i in 0..primes_under_1000.len() {
        let mut working_sum = primes_under_1000[i];
        for j in i+1..primes_under_1000.len() {
            working_sum += primes_under_1000[j];
            if working_sum >= 1000 {
                break;
            }
            if pset.is_prime(working_sum) && (j - i + 1 > x_max) {
                x_max = j+2;
                first_prime = i;
                y = working_sum;
            }
        }
    }

    println!("{} consecutive primes with sum {}: ", x_max, y);
    for (_, prime) in pset.iter().enumerate().skip(first_prime).take(x_max) {
        print!("{}" , prime);
        println!("");
    }
}