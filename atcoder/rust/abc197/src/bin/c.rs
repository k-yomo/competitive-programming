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
        a: [i128; n],
    }

    let mut min = 2_i128.pow(31);
    for bit_flag in 0..(1 << n - 1) {
        let mut sep = vec![];
        for i in 0..n - 1 {
            if bit_flag & (1 << i) != 0 {
                sep.push(i);
            }
        }
        let mut cur_or = 0_i128;
        let mut cur_xor = 0_i128;
        let mut cur_idx = 0;
        for (i, num) in a.iter().enumerate() {
            cur_or |= num;
            if cur_idx < sep.len() && i == sep[cur_idx] {
                cur_xor ^= cur_or;
                cur_idx += 1;
                cur_or = 0;
            }
        }
        chmin!(min, cur_xor ^ cur_or);
    }

    println!("{}", min);
}
