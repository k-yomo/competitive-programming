#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        c: Chars,
    }

    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut d_count = 0;
    for choice in c {
        match choice {
            '1' => a_count += 1,
            '2' => b_count += 1,
            '3' => c_count += 1,
            '4' => d_count += 1,
            _ => (),
        }
    }

    println!("{} {}",
             max(max(a_count, b_count), max(c_count, d_count)),
             min(min(a_count, b_count), min(c_count, d_count))
    );
}
