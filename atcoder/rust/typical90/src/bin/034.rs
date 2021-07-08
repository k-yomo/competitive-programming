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
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut num_map = HashMap::new();
    let mut uniq_count = 0;
    let mut cur_len = 0;
    let mut max_len = 0;
    for i in 0..n {
        let count = num_map.entry(a[i]).or_insert(0);
        *count += 1;
        if *count == 1 {
            uniq_count += 1;
        }
        cur_len += 1;
        if uniq_count > k {
            loop {
                let count = num_map.entry(a[i - (cur_len - 1)]).or_insert(0);
                *count -= 1;
                cur_len -= 1;
                if *count == 0 {
                    uniq_count -= 1;
                    break;
                }
            }
        }
        max_len = std::cmp::max(cur_len, max_len);
    }

    println!("{}", max_len);
}
