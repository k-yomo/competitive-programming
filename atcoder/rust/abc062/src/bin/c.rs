#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use proconio::*;

#[macro_export]
macro_rules! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
#[macro_export]
macro_rules! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
#[macro_export]
macro_rules! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
#[macro_export]
macro_rules! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    if h % 3 == 0 || w % 3 == 0 {
        return println!("0");
    }

    let mut min_diff = std::usize::MAX;
    if h > 2 {
        for i in 1..h {
            let a = i * w;
            let b = (h - i + 1) / 2 * w;
            let c = (h - i) / 2 * w;
            chmin!(min_diff, max!(a, b, c) - min!(a, b, c));
        }
    }
    if w > 2 {
        for i in 1..w {
            let a = i * h;
            let b = (w - i + 1) / 2 * h;
            let c = (w - i) / 2 * h;
            chmin!(min_diff, max!(a, b, c) - min!(a, b, c));
        }
    }
    for i in 1..h {
        let a = i * w;
        let b = (h - i) * ((w + 1) / 2);
        let c = (h - i) * (w / 2);
        chmin!(min_diff, max!(a, b, c) - min!(a, b, c));
    }
    for i in 1..w {
        let a = i * h;
        let b = (w - i) * ((h + 1) / 2);
        let c = (w - i) * (h / 2);
        chmin!(min_diff, max!(a, b, c) - min!(a, b, c));
    }

    println!("{}", min_diff);
}
