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
        c: usize,
        mut nml: [[usize; 3]; c],
    }

    let sorted_nml = nml
        .iter()
        .map(|nums| nums.iter().sorted().collect::<Vec<&usize>>())
        .collect::<Vec<Vec<&usize>>>();

    println!(
        "{}",
        sorted_nml.iter().map(|nums| nums[0]).max().unwrap()
            * sorted_nml.iter().map(|nums| nums[1]).max().unwrap()
            * sorted_nml.iter().map(|nums| nums[2]).max().unwrap()
    );
}
