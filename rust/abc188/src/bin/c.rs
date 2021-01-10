#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: u32,
    }

    if n == 1 {
        input! {
            a: usize,
            b: usize,
        }
        println!("{}", if a > b { 2 } else { 1 });
        return;
    }

    let count = 2_i32.pow(n);
    let mut l = vec![];
    let mut r = vec![];
    for i in 1..(count + 1) {
        input! {
            ai: u128,
        }
        if i > count / 2 { l.push((i, ai));} else { r.push((i, ai)) }
    }
    l.sort_by_key(|x| x.1);
    r.sort_by_key(|x| x.1);
    println!("{}", if l.last().unwrap().1 > r.last().unwrap().1 { r.last().unwrap().0 } else { l.last().unwrap().0 });
}
