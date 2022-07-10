#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::{BufRead, Write};
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use whiteread::parse_string;

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let a_nums = a.iter().
        map(|s| s.iter().map(|c| c.to_string().parse().unwrap()).collect::<Vec<usize>>()).
        collect::<Vec<Vec<usize>>>();

    let n = n as i64;
    let mut max_total  = 0usize;
    for y in 0..n {
        for x in 0..n {
            for (y_diff, x_diff) in vec![(1, 0), (0, 1), (1, 1), (0, -1), (-1, 1), (-1, 0), (1, -1), (-1, -1)].iter() {
                let mut total = 0usize;
                let mut cur_y = y;
                let mut cur_x = x;
                for i in 0..n {
                    total *= 10;
                    total += a_nums[cur_y as usize][cur_x as usize];
                    cur_y = (cur_y + n + y_diff) % n;
                    cur_x = (cur_x + n + x_diff) % n;
                }
                if total > max_total {
                    max_total = total;
                }
            }
        }
    }

    println!("{}", max_total);
}
