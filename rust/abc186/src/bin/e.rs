#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        t: usize,
        (nsk): [(usize, usize, usize); t],
    }

    for i in 0..t {
        let (n, s, k) = nsk[i];
        let mut count = 0;
        let mut pos_map: HashMap<usize, bool> = HashMap::new();
        loop {
            if *pos_map.entry((k * &count) % n).or_default() {
                println!("-1");
                break
            }
            pos_map.insert((k * &count) % n, true);
            if (n - s) == (k * &count) % n {
                println!("{}", count);
                break
            }
            count += 1;
        }
    }
}
