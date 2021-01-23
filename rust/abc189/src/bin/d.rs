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
        s: [String; n],
    }


    let mut count = 0;
    let mut or_count = 1_u32;
    for (i, op) in s.iter().rev().enumerate() {
        if op == "AND" {
            count += 1;
            if or_count > 0 {
                count += 2_i128.pow(or_count);
            }
            or_count = 0;
        } else {
            or_count += 1;
        }
    }
    if or_count > 0 {
        count += 2_i128.pow(or_count);
    }
    println!("{}", count - 1);
}
