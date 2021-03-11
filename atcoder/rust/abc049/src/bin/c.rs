#![allow(unused_imports)]
use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

fn main() {
    input! {
        mut s: String,
    }

    s = ["eraser", "erase", "dreamer", "dream"]
        .iter()
        .fold(s, |st, w| st.replace(w, ""));

    if s.is_empty() {
        println!("YES")
    } else {
        println!("NO")
    }
}
