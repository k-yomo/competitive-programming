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
        n: usize,
    }

    let mut pow_map = HashMap::new();

    let mut i = 2;
    while i * i <= n {
        let mut c = 2;
        loop  {
            let (v, of) = i.overflowing_pow(c as u32);
            if of || v > n {
                break
            }
            pow_map.insert(v, true);
            c += 1;
        }
        i += 1;
    }

    println!("{}", n - pow_map.len());
}
