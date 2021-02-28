#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: i64, a: i64, b: i64,
    }

    let ans = mod_pow(2, n, MOD) - cmb_mod(n, a) - cmb_mod(n, b) - 1;
    println!("{}", (ans + 2 * MOD) % MOD);
}

const MOD: i64 = 1000000007;

fn cmb_mod(left: i64, right: i64) -> i64 {
    if right == 0 {
        return 1;
    }
    let mut v = left;
    let mut div = right;
    for i in 1..right {
        v = v * (left - i) % MOD;
        div = div * (right - i) % MOD;
    }
    div = mod_pow(div, MOD - 2, MOD);
    return v * div % MOD;
}


fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}