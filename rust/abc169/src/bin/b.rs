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

    if *a.iter().min().unwrap() == 0 {
        println!("0");
        return
    }

    let border = 10_usize.pow(18);
    let mut product = 1;
    for num in a {
        if product * num > border || product * num / num != product {
            println!("-1");
            return
        }
        product *= num;
    }

    println!("{}", product);
}
