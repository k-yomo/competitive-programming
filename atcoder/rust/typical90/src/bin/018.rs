#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::f64::consts::PI;
use std::io::Write;
use std::ops::Bound::*;

use itertools::__std_iter::once;
use itertools::*;
use itertools_num::ItertoolsNum;
use libm::{atan2, cos, sin};
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
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q],
    }

    // t/4 == 0, -L/2, L/2)
    // t/2 == 0, 0, L)
    // 3t/4 == 0, L/2, L/2)
    // t/2 = 半周
    // t/4 = 4分の1週
    // t / t = t分の1周

    // (x-a)^2+(y-b)^2=r^2
    // (y-b)^2=r^2-(x-a)^2
    // y^2 - 2yb + b^2=r^2-(x-a)^2

    let radius = l / 2.0;
    for i in e {
        let cur_i = i % t;
        let cur_angle = 360.0 / t * cur_i;
        let cur_y = radius * cos(cur_angle * (PI / 180.0));
        let cur_z = radius * sin(cur_angle * (PI / 180.0));
        println!(
            "{}",
            atan2(cur_z, (x.powf(2.0) + (cur_y - y).powf(2.0)).sqrt()) / PI * 180.0
        )
    }
}
