#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: String,
        b: String,       
    }

    if a.len() > b.len() {
        println!("GREATER");
        return
    } else if a.len() < b.len() {
        println!("LESS");
        return
    }

    let mut a_chars = a.chars();
    let mut b_chars = b.chars();
    for i in 0..a.len() {
        let a_num = a_chars.nth(i);
        let b_num = b_chars.nth(i);
        if a_num == b_num { 
            continue
        }
        println!("{}", if a_num > b_num { "GREATER" } else { "LESS" });
        return
    }
    println!("EQUAL");
}
