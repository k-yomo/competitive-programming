#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

#[macro_export]
macro_rules! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
#[macro_export]
macro_rules! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[macro_export]
macro_rules! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let count = std::cmp::min(8, n);
    let mut mods: Vec<Vec<usize>> = vec![vec![]; 256];
    for bit_flag in 0..(1 << count) {
        let mut nums: Vec<usize> = vec![];
        let mut m = 0;
        for i in 0..count {
            if bit_flag & 1 << i != 0 {
                nums.push(i);
                m += a[i];
                m %= 200;
            }
        }
        if !mods[m].is_empty() {
            println!("Yes");
            println!(
                "{} {}",
                mods[m].len(),
                mods[m].iter().map(|x| (x + 1).to_string()).join(" ")
            );
            println!(
                "{} {}",
                nums.len(),
                nums.iter().map(|x| (x + 1).to_string()).join(" ")
            );
            return;
        } else {
            mods[m] = nums;
        }
    }

    println!("No");
}
