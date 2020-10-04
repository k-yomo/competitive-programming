#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        m: usize,
        roads: [(usize, usize); m]
    }

    let mut city_road_counts = vec![0; n];
    for (a,b) in roads {
        city_road_counts[a-1] += 1;
        city_road_counts[b-1] += 1;
    }

    city_road_counts.iter().for_each(|x| println!("{}", x));
}
