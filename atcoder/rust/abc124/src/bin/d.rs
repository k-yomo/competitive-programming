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
        n: usize, k: usize,
        s: Chars,
    }

    let mut groups = vec![];
    let mut count = 0;
    let mut is_zero = s[0] == '0';
    for c in s {
        if (c == '0') == is_zero {
            count += 1;
        } else {
            groups.push((count, is_zero));
            is_zero = !is_zero;
            count = 1;
        }
    }

    groups.push((count, is_zero));

    let mut zero_count = 0;
    let mut len = 0;
    let mut max_len = 0;
    let mut l = 0;
    for (i, (count, is_zero)) in groups.iter().enumerate() {
        len += count;
        if *is_zero {
            zero_count += 1;
        }
        if zero_count <= k {
            max_len = std::cmp::max(max_len, len);
        } else {
            while zero_count > k {
                if groups[l].1 {
                    zero_count -= 1;
                }
                len -= groups[l].0;
                l += 1;
            }
        }
    }

    println!("{}", std::cmp::max(max_len, len))
}
