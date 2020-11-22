#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    if x1 == x2 && y1 == y2 {
        println!("0");
        return;
    }
    // 範囲内
    if (x1 - x2).abs() + (y1 - y2).abs() <= 3 {
        println!("1");
        return;
    }
    // 筋
    if (x1 - x2).abs() == (y1 - y2).abs() {
        println!("1");
        return;
    }
    // 縦横
    for i in -3..4 {
        // 筋
        if ((x1 + i) - x2).abs() == (y1 - y2).abs() {
            println!("2");
            return;
        }
        if (x1 - x2).abs() == ((y1 + i) - y2).abs() {
            println!("2");
            return;
        }
        // 範囲内
        if ((x1 + i) - x2).abs() + (y1 - y2).abs() <= 3 {
            println!("2");
            return;
        }
        if (x1 - x2).abs() + ((y1 + i) - y2).abs() <= 3 {
            println!("2");
            return
        }
    }
    // 筋違い
    if (x1 - x2).abs() % 2 == (y1 - y2).abs() % 2 {
        println!("2");
        return;
    }
    println!("3");
}
