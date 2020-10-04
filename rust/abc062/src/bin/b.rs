#![allow(unused_imports)]
use proconio::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        h: usize,        
        w: usize,
        image: [String; h],
    }

    println!("{}", "#".repeat(w+2));
    for row in image {
        print!("#");
        for cell in row.chars() {
            print!("{}", cell);
        }
        println!("#");
    }
    println!("{}", "#".repeat(w+2));
}
