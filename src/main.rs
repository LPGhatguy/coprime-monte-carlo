extern crate rand;

use rand::Rng;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }

    a
}

fn are_coprime(a: u64, b: u64) -> bool {
    1 == gcd(a, b)
}

fn main() {
    let max_value = u64::max_value();
    let num_samples: usize = 10_000_000;

    let mut rng = rand::thread_rng();
    let mut num_coprime: usize = 0;

    println!("Calculating...");

    for _ in 0..num_samples {
        let a: u64 = rng.gen_range(1, max_value);
        let b: u64 = rng.gen_range(1, max_value);

        if are_coprime(a, b) {
            num_coprime += 1;
        }
    }

    println!("Results:");
    println!("{} coprime values out of {} total", num_coprime as usize, num_samples);
    println!("{:.5}% coprime", 100.0 * (num_coprime as f32) / (num_samples as f32));
    println!("1/zeta(2): {}", 6.0 / std::f64::consts::PI.powi(2));
}
