#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize,        
        b: usize,        
        c: usize,        
    }

    let mut cur_num = a;
    for _ in 0..b {
        if cur_num % b == c {
            println!("YES");
            return
        }
        cur_num += a;
    }
    println!("NO");
}
