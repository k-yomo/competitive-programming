#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;

fn main() { 
    input! {
        n: usize,
        mut a: [i128; n],
    }
    a.sort();
    let a_acc = a.iter().cumsum().collect::<Vec<i128>>();
    let mut abs_sum = 0;
    for i in 0..n {
        abs_sum += (a_acc[a_acc.len()-1] - a_acc[i]) - a[i] * (n - 1 - i) as i128;
    }
    println!("{}", abs_sum);
}
