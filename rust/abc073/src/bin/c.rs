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
        a: [usize; n],
    }

    let mut num_map = HashMap::new();
    for num in a {
        let exist = num_map.entry(num).or_insert(false);
        *exist = !*exist;
    }
    println!("{}", num_map.iter().filter(|(_, &v)| { v }).count());
}
