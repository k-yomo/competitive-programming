#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut step_height_sum = 0;
    let mut last_height = a[0];
    for &height in a[1..].iter() {
        let mut cur_height = height;
        if cur_height < last_height {
            step_height_sum += last_height - cur_height;
        } else {
            last_height = cur_height;
        }
    }
    println!("{}", step_height_sum);
}
