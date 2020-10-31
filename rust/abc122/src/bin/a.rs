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
        b: char,
    }

    match b {
        'A' => println!("T"),
        'T' => println!("A"),
        'C' => println!("G"),
        'G' => println!("C"),
        _ => {},
    }
}
