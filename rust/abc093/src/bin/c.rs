#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

fn main() {
    input! {
        mut nums: [usize; 3],
    }

    nums.sort();
    let (mut min, mut mid, mut max) = (nums[0], nums[1], nums[2]);
    let mut count = 0;
    count += max - mid;
    min += count;
    count += (max - min + 1) / 2;
    if (max - min) % 2 != 0 {
        count += 1;
    }

    println!("{}", count);
}
