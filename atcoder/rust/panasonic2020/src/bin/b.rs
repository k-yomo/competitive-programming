#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        h: usize,
        w: usize,
    }

	if h == 1 || w == 1 {
		println!("1");
		return
    }

    let mut ans = h * w / 2;
    if h * w % 2 != 0 {
        ans += 1;
    }

    println!("{}", ans);
}
