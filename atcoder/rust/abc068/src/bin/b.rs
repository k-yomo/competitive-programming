#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,        
    }

    if n < 2 {
        println!("1");
        return
    }

    let mut num = 2;
    loop {
        if num * 2 > n {
            break;
        }
        num *= 2;
    }
    println!("{}", num);
}
