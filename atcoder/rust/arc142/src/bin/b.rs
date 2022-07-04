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
        n: usize,
    }

    for i in 1..=n {
        let mut nums = vec![];
        let mut reverse = false;
        let mut i = 0;
        let mut count = 0;
        while count < n {
            if reverse {
                print!("{} ", nums[n-i-1]);
                i += 1;
            } else {
                print!("{} ", nums[i]);
            }
            count += 1;
            reverse = !reverse;
        }
        println!(nums.map(|i| i.to_string()).jon(" "))
    }
}


