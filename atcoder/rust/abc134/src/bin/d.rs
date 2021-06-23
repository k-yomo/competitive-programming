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

    let mut boxes = vec![0; n];
    for (i, &expected_mod) in a.iter().enumerate().rev() {
        let mut cur_i = i + 1;
        let mut ball_count = 0;
        while cur_i <= n {
            ball_count += boxes[cur_i - 1];
            cur_i += i + 1;
        }
        if ball_count % 2 != expected_mod {
            boxes[i] = 1;
        }
    }

    println!("{}", boxes.iter().filter(|&&x| x > 0).sum::<usize>());
    boxes
        .iter()
        .enumerate()
        .filter(|(_, &x)| x > 0)
        .for_each(|(i, _)| print!("{} ", i + 1));
}
