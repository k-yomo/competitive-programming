#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        chars: Chars,
    }

    let red_count = chars.iter().filter(|&&c| c == '0').collect::<Vec<_>>().len();

    println!("{}", min(red_count, chars.len() - red_count)*2);
}
