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
        n: usize,
        mut a: [usize; n],
    }

    let mut num_map = HashMap::new();
    for (i, &num) in &a.iter().enumerate() {
        num_map.insert(num, i+1);
    }

    println!("{}", (1..n+1).map(|i| num_map.get(&i).unwrap()).join(" "));
}
