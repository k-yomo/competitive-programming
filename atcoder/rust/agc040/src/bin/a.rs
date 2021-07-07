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

    let mut nums = vec![0; s.len() + 1];
    for i in 1..=s.len() {
        if s[i - 1] == '<' {
            nums[i] = nums[i - 1] + 1;
        }
    }
    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            nums[i] = nums[i].max(nums[i + 1] + 1);
        }
    }
    println!("{}", nums.iter().sum::<usize>())
}
