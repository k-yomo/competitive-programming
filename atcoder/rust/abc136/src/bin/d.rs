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
        s: Chars,
    }

    let mut child_nums = vec![0; s.len()];
    let mut count = 0;
    for i in 0..s.len() - 1 {
        if s[i] == 'L' {
            continue;
        }
        count += 1;
        if s[i] == 'R' && s[i + 1] != 'R' {
            child_nums[i] += (count + 1) / 2;
            child_nums[i + 1] += count / 2;
            count = 0;
        }
    }
    for i in (1..s.len()).rev() {
        if s[i] == 'R' {
            continue;
        }
        count += 1;
        if s[i] == 'L' && s[i - 1] != 'L' {
            child_nums[i] += (count + 1) / 2;
            child_nums[i - 1] += count / 2;
            count = 0;
        }
    }

    println!("{}", child_nums.iter().map(|i| i.to_string()).join(" "))
}
