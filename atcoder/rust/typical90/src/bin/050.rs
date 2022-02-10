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

const Mod: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let mut fact = vec![1; 1000000];
    let mut fact_inv = vec![1; 1000000];
    let mut inv = vec![1; 1000000];
    for i in 2..100000 {
        fact[i] = fact[i - 1] * i % Mod;
        inv[i] = Mod - inv[Mod % i] * (Mod / i) % Mod;
        fact_inv[i] = fact_inv[i - 1] * inv[i] % Mod;
    }

    let mut ans = 0;
    let max_l_num = n / l;
    for i in 0..=max_l_num {
        let n = (n - i * l) + i;
        ans = (ans + fact[n] * (fact_inv[i] * fact_inv[n - i] % Mod)) % Mod;
    }

    println!("{}", ans)
}

