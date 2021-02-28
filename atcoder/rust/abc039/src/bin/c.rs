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
        s: String,
    }

    println!("{}",
         match s.find("WWBWBWW").unwrap() {
             0 => "Si",
             2 => "La",
             4 => "So",
             6 => "Fa",
             7 => "Mi",
             9 => "Re",
             11 => "Do",
             _ => unreachable!(),
         },
    );
}
