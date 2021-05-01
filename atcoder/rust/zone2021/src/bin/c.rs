#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use proconio::marker::*;
use proconio::*;
use superslice::*;

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
        n: usize,
        mut abcde: [(u128, u128,u128,u128,u128); n],
    }

    abcde.sort_by(|a, b| (a.0 + a.1 + a.2 + a.3 + a.4).cmp(&(b.0 + b.1 + b.2 + b.3 + b.4)));

    let mut max = 0;
    for cmb in ((if n > 500 { n - 300 } else { 0 })..n).combinations(3) {
        let x = abcde[cmb[0]];
        let y = abcde[cmb[1]];
        let z = abcde[cmb[2]];
        let m = min!(
            max!(x.0, y.0, z.0),
            max!(x.1, y.1, z.1),
            max!(x.2, y.2, z.2),
            max!(x.3, y.3, z.3),
            max!(x.4, y.4, z.4),
        );
        max = std::cmp::max(max, m);
    }

    println!("{}", max)
}
