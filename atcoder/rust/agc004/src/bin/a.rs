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
        mut nums: [usize; 3],
    }

    nums.sort();
    println!(
        "{}",
        nums[0] * nums[1] * ((nums[2] + 1) / 2) - nums[0] * nums[1] * (nums[2] / 2)
    )
}
