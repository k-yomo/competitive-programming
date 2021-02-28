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
        mut n: usize, k: usize,
    }

    for i in 0..k {
        let mut n_asc = n.to_string().chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();
        n_asc.sort();
        let mut m = n_asc.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        n = m.iter().rev().join("").parse::<usize>().unwrap() - m.iter().join("").parse::<usize>().unwrap();
    }

    println!("{}", n);
}
