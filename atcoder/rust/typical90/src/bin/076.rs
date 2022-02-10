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
        n: usize,
        a: [usize; n],
    }

    let total_size = a.iter().sum::<usize>();
    let mut cur_size = 0;
    let (mut l, mut r) = (0, 0);
    let a2 = a.iter().chain(a.iter()).map(|&i| i).collect::<Vec<usize>>();
    for (i, size) in a2.iter().enumerate() {
        cur_size += size;
        r = i;
        while cur_size > total_size / 10 {
            cur_size -= a2[l];
            l += 1;
        }
        if l <= r && cur_size == total_size / 10 {
            return println!("Yes");
        }
    }

    println!("No")
}
