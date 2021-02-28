#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use permutohedron::factorial;

fn main() { 
    input! {
        l: u128,
    }

    println!("{}", (l-11..=l-1).product::<u128>() / factorial(11) as u128);
}
