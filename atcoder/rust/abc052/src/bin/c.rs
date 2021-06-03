#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize,
    }

    let primes = sieve_of_eratosthenes(n);
    let mut prime_count_map = HashMap::new();
    for i in 1..=n {
        for p in primes.iter() {
            let mut i = i;
            while i % p == 0 {
                i /= p;
                *prime_count_map.entry(p).or_insert(0) += 1;
            }
        }
    }

    let mut count: usize = 1;
    for (_, c) in prime_count_map {
        if c > 0 {
            count *= c + 1;
            count %= 1_000_000_007;
        }
    }
    println!("{}", count);
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut spf = vec![None; n + 1];
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..n + 1 {
        if is_prime[i] {
            primes.push(i);
            spf[i] = Some(i);
        }

        for prime in &primes {
            if i * prime >= n + 1 || prime > &spf[i].unwrap() {
                break;
            }

            is_prime[i * prime] = false;

            spf[i * prime] = Some(*prime);
        }
    }
    primes
}
