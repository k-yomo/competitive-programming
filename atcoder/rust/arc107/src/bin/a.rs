#![allow(unused_imports)]

use itertools::*;
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
    }

    // (1 * a * b) + (1 * (a+1) * b) + (1 * (a+2) * b) + (1 * (a+3) * b)
    // ab(1) + ab(2) + ab(3) ... + ab(c)
    // ab(1..c)
    // 1b(a/2(a+1)) * ()
    const m: i128 = 998244353;
    let mut a_sum = a % m / 2 % m * (a + 1) % m;
    if a_sum == 0 { a_sum = 1 };
    let mut b_sum = b % m / 2 % m * (b + 1) % m;
    if b_sum == 0 { b_sum = 1 };
    let mut c_sum = c % m / 2 % m * (c + 1) % m;
    if c_sum == 0 { c_sum = 1 };
    println!("a: {}, b: {}, c: {}", a_sum * a, b_sum * b, c_sum * c);
    println!("{}", a_sum * a * b_sum * b * c_sum * c / (a+b+c) % m);
}
