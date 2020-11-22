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
        a: f64,
        b: f64,
        c: f64,
    }

    let mut dp = vec![Vec<Vec<[f64]>: 100];
    db[0][0][0] = 0;

    let ap = (100_f64 - a)  * (a/(a+b+c));
    let bp = (100_f64 - b)  * (b/(a+b+c));
    let cp = (100_f64 - c)  * (c/(a+b+c));
    println!("{}", ap * bp * cp);
    // println!("{}", (100_f64 - a)  * (a/(a+b+c)) * (100_f64-b) * (b/(a+b+c)) * (100_f64-c) * (c/(a+b+c)));
}
