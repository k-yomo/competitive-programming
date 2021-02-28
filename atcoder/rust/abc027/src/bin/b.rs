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
        mut a: [usize; n],
    }

    if a.iter().sum::<usize>() % n != 0 {
        println!("-1");
        return
    }
    let each = a.iter().sum::<usize>() / n;

    let mut count = 0;
    for i in 1..a.len() {
        if a[0..i].iter().sum::<usize>() != each * i {
            count += 1;
        }
    }

    println!("{}", count);
}
