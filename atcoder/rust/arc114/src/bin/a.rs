#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut min_num = std::usize::MAX;
    let primes = sieve_of_eratosthenes(50);
    for bit_flag in 0..(1 << primes.len()) {
        let mut num = 1;
        for i in 0..primes.len() {
            if bit_flag & (1 << i) != 0 {
                num *= primes[i]
            }
        }
        if x.iter().all(|&i| num::integer::gcd(i, num) != 1) {
            min_num = std::cmp::min(min_num, num);
        }
    }

    println!("{}", min_num);
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
