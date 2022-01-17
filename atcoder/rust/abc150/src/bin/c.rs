#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let perms = (1..=n).permutations(n).collect::<Vec<Vec<usize>>>();
    let a = perms.iter().position(|perm| perm.eq(&p)).unwrap() as i64;
    let b = perms.iter().position(|perm| perm.eq(&q)).unwrap() as i64;
    println!("{}", (a - b).abs());
}
