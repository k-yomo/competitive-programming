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
        h: usize, w: usize,
        p: [[usize; w]; h],
    }

    let mut max_count = 0;
    for bit_flag in 0..(1 << h) {
        let mut num_count_map = HashMap::new();
        for j in 0..w {
            let mut num = 0;
            let mut num_count = 0;
            for i in 0..h {
                if bit_flag & (1 << i) != 0 {
                    if num == 0 {
                        num = p[i][j]
                    } else if num != p[i][j] {
                        num_count = 0;
                        break;
                    }
                    num_count += 1;
                }
            }
            *num_count_map.entry(num).or_insert(0) += num_count;
        }
        let m = num_count_map.iter().max_by_key(|entry| entry.1).unwrap();
        if *m.1 > max_count {
            max_count = *m.1;
        }
    }

    println!("{}", max_count);
}
