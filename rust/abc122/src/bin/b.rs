#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

fn main() { 
    input! {
        s: Chars,
    }

    let mut max_len = 0;
    let mut cur_len = 0;
    for c in s {
        match c {
            'A' => cur_len += 1,
            'C' => cur_len += 1,
            'G' => cur_len += 1,
            'T' => cur_len += 1,
            _ => {
                if cur_len > max_len {
                    max_len = cur_len;
                }
                cur_len = 0;
            },
        }
    }
    if cur_len > max_len {
        max_len = cur_len;
    }
    println!("{}", max_len);
}
