#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        mut s: [usize; n],
    }

    let mut total = s.iter().sum::<usize>();
    if total % 10 != 0 {
        println!("{}", total);
        return
    }

    s.sort();
    for num in s {
        if num % 10 != 0 {
            total -= num;
            println!("{}", total);
            return
        }
    }
    println!("0");
}
