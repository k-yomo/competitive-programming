#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use std::ops::Index;

use itertools::*;
use nalgebra::DimAdd;
use proconio::*;
use proconio::marker::*;
use whiteread::parse_string;

fn main() {
    input! {
        s: Chars,
    }

    let mut sum = 0;
    for bit_flag in 0..(1 << s.len() - 1) {
        let mut strs: Vec<String> = vec![];
        let mut last_i = 0;
        for i in 0..(s.len() - 1) {
            if bit_flag & (1 << i) != 0 {
                strs.push(s[last_i..i + 1].iter().collect());
                last_i = i + 1;
            }
        }
        strs.push(s[last_i..].iter().collect());
        sum += strs.iter().map(|str| parse_string::<usize>(str).unwrap()).sum::<usize>();
    }

    println!("{}", sum);
}
