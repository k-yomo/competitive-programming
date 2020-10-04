#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut pushed_button: HashMap<usize, bool> = HashMap::new();
    let mut cur_button = 1;
    let mut count = 0;
    loop {
        pushed_button.insert(cur_button, true);
        cur_button = a[cur_button - 1];
        count += 1;
        if cur_button == 2 {
            println!("{}", count);
            return
        }
        if *pushed_button.entry(cur_button).or_insert(false) {
            println!("-1");
            return
        }
    }
}
