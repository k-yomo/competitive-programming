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
        (csf): [(usize, usize, usize); n-1],
    }

    for i in 0..(n - 1) {
        let mut elapsed_sec = csf[i].0 + csf[i].1;
        for j in (i+1)..(n - 1) {
            elapsed_sec += csf[j].0 + if elapsed_sec <= csf[j].1 {
                csf[j].1 - elapsed_sec
            } else {
               (csf[j].2 - (elapsed_sec - csf[j].1) % csf[j].2 ) % csf[j].2
            }
        }
        println!("{}", elapsed_sec);
    }
    println!("0");
}
