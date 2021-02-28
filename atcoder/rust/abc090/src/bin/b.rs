#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        a: usize,
        b: usize,
    }

    let mut palindrome_count = 0;
    for i in a..b+1 {
        let chars: Vec<char> = i.to_string().chars().collect();
        let mut is_palindrome = true;
        for j in 0..(chars.len()/2) {
            if chars[j] != chars[chars.len()-1-j] {
                is_palindrome = false;
                break;
            }
        }
        if is_palindrome {
            palindrome_count+= 1;
        }
    }

    println!("{}", palindrome_count);
}
