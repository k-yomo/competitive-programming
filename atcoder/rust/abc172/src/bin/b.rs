#![allow(unused_imports)]
use itertools::*;
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    println!("{}", (0..s.len()).filter(|&i| s[i] != t[i]).count());
}
