#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut atcoder_colors = vec![0; 9];
    for rate in a {
        atcoder_colors[std::cmp::min(rate / 400, 8)] += 1;
    }

    let color_count = atcoder_colors[0..8].iter().filter(|&&num| num > 0).count();
    println!("{} {}", std::cmp::max(color_count, 1), color_count + atcoder_colors[8]);
}
