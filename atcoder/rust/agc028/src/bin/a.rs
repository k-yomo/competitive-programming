#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use num_integer::lcm;
use proconio::marker::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n: usize, m: usize,
        s: Chars,
        t: Chars,
    }

    let length = lcm(n, m);

    let mut idx_char_map = HashMap::new();
    for i in 0..n {
        idx_char_map.insert(i * length / n, s[i]);
    }
    for i in 0..m {
        let v = idx_char_map.get(&(i * length / m));
        match v {
            Some(v) => {
                if v != &t[i] {
                    return println!("-1");
                }
            }
            None => {}
        }
    }

    println!("{}", length);
}
