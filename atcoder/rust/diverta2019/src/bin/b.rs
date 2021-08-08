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
        r: usize, g: usize, b: usize, n: usize,
    }

    let mut count = 0;
    for i in 0..=n / r {
        for j in 0..=n / g {
            let rg = r * i + g * j;
            if rg > n {
                break;
            }
            if (n - rg) % b == 0 {
                count += 1
            }
        }
    }

    println!("{}", count)
}
