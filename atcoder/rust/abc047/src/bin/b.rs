#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        mut w: usize,
        mut h: usize,
        n: usize,
        dots: [(usize, usize, usize); n],
    }

    let mut x_left = 0;
    let mut x_right = w;
    let mut y_top = h;
    let mut y_bottom = 0;
    for dot in dots {
        let x = dot.0;
        let y = dot.1;
        let a = dot.2;
        match a {
            1 => {
                if x > x_left { x_left = x }
            },
            2 => {
                if x < x_right { x_right = x }
            },
            3 => {
                if y > y_bottom { y_bottom = y }
            },
            4 => {
                if y < y_top { y_top = y }
            },
            _ => continue,
        }
    }

    if x_left >= x_right || y_bottom >= y_top {
        println!("0");
        return
    }

    println!("{}", (x_right - x_left) * (y_top - y_bottom));
}
