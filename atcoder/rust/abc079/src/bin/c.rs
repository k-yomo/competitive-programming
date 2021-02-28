#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;
use whiteread::parse_string;

fn main() { 
    input! {
        abcd: Chars
    }

    for bit_flag in 0..(1 << 3) {
        let mut sum = parse_string::<i64>(&abcd[0].to_string()).unwrap();
        let mut formula = vec![abcd[0]];
        for i in 0..3 {
            if bit_flag & (1 << i) != 0 {
                sum += parse_string::<i64>(&abcd[i+1].to_string()).unwrap();
                formula.append(&mut vec!['+', abcd[i+1]]);
            } else {
                sum -= parse_string::<i64>(&abcd[i+1].to_string()).unwrap();
                formula.append(&mut vec!['-', abcd[i+1]]);
            }
        }
        if sum == 7 {
            println!("{}=7", formula.iter().collect::<String>());
            return
        }
    }
}
