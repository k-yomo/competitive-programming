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
        h: usize, w: usize,
        s: [Chars; h]
    }

    let mut corner_count = 0;
    for y in 1..h {
        for x in 1..w {
            let mut black_num = vec![s[y][x], s[y][x-1], s[y-1][x-1], s[y-1][x]].iter().filter(|x| **x == '#').count();
            if black_num == 1 || black_num == 3 {
                corner_count += 1;
            }
        }
    }

    println!("{}", corner_count);
}
