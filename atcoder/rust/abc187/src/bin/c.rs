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
        n: usize,
        s: [String; n],
    }

    let mut str_map: HashMap<String, bool> = HashMap::new();
    for str in s.iter() {
        str_map.insert(str.to_string(), true);
    }
    for str in s {
        if *str_map.entry(format!("!{}", str)).or_default() {
            println!("{}", str);
            return
        };
    }

    println!("satisfiable");
}
