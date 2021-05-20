#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        n: usize, k: usize,
    }

    if k % 2 == 0 {
        // a ≡ b ≡ c ≡ 0 mod K || a ≡ b ≡ c ≡ k/2 mod K
        println!("{}", (n / k).pow(3) + ((n + k / 2) / k).pow(3))
    } else {
        // a ≡ b ≡ c ≡ 0 mod K
        println!("{}", (n / k).pow(3));
    }
}
