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
        p: [usize; n],
    }

    let mut cur_min = 0;
    let mut exist_num_map: HashMap<usize, bool> = HashMap::new();
    for num in p {
        exist_num_map.insert(num, true);
        while *exist_num_map.entry(cur_min).or_default() {
            cur_min += 1;
        }
        println!("{}", cur_min);
    }
}
