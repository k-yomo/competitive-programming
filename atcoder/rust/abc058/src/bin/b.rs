#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        o: String,
        e: String,       
    }
    
    for i in 0..e.len() {
        print!("{}", o.chars().nth(i).unwrap());
        print!("{}", e.chars().nth(i).unwrap());
    }
    if o.len() > e.len() {
        print!("{}", o.chars().nth(o.len()-1).unwrap());
    }
    println!("");
}
