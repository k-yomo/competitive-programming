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
        s: Chars,
    }

    let mut last_slime = s[0];
    let mut slime_count = 1;
    for &slime in s[1..].iter() {
        if slime != last_slime {
            slime_count += 1;
            last_slime = slime;
        }
    }

    println!("{}", slime_count);
}
