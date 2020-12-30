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
        (n, m): (usize, usize),
        py: [(usize, usize); m],
    }

    let mut sorted_py = py.iter().enumerate().map(|(i, (p, y))| (i, p, y)).collect::<Vec<(usize, &usize, &usize)>>();
    sorted_py.sort_by(|a, b| a.2.cmp(&b.2));
    let (mut order, mut cur_p) = (1, sorted_py[0].1);
    let mut city_birth_order_map = HashMap::new();
    let mut p_order = HashMap::new();
    for (i, p, y) in sorted_py {
        let order = p_order.entry(p).or_insert(0);
        *order += 1;
        city_birth_order_map.insert(i, *order);
    }
    for (i, (p, y)) in py.iter().enumerate() {
        let order = city_birth_order_map.get(&i).unwrap();
        println!("{}",  format!("{:0>6}{:0>6}", p, order));
    }
}
