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
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    }

    for y in 0..n-m+1 {
        for x in 0..n-m+1 {
            let mut img_match = true;
            for (i, chars) in b.iter().enumerate() {
                for (j, &c) in chars.iter().enumerate() {
                    if a[y+i][x+j] != c {
                        img_match = false;
                        break;
                    }
                }
                if !img_match {
                    break;
                }
            }
            if img_match {
                println!("Yes");
                return
            }
        }
    }
    println!("No");
}
