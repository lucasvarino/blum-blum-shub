// Blum-blum shub

use rand::Rng;

fn get_primes(limit: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; limit as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..limit {
        if is_prime[i as usize] {
            primes.push(i);
            let mut j = i * i;
            while j < limit {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    primes
}

fn blum_blum_shub(n: u64, p: u64, q: u64, seed: u64) -> u64 {
    let m = p * q;
    let mut x = seed;
    let mut result = 0;
    for _ in 0..n {
        x = x * x % m;
        result = result << 1 | x & 1;
    }
    result
}

fn main() {
    let primes = get_primes(1000);
    let p = primes[primes.len() - 1];
    let q = primes[primes.len() - 2];
    let seed = rand::thread_rng().gen_range(0..p * q);
    let n = 10;
    let result = blum_blum_shub(n, p, q, seed);
    let result_in_bit_string = format!("{:b}", result);
    println!("Blum-blum shub: {}", result);
    println!("P = {}, Q = {}", p, q);
    println!("S: {}", seed);
    println!("Result in bit string: {}", result_in_bit_string);
}
