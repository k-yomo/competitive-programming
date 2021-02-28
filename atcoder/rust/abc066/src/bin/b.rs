#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        s: String,        
    }

    let mut i = if s.len() %2 == 0 { 2 } else { 1 };
    loop {
        let a = String::from_utf8(s.as_bytes()[0..s.len()-i].to_vec()).unwrap();
        let (first, second) = a.split_at(a.len()/2);
        if first  == second {
            println!("{}", s.len() - i);
            return
        }
        i += 2;
    }
}
