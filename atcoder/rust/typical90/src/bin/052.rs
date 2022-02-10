#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

const Mod: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }

    //

    let dice_sums: Vec<usize> = a.iter().map(|dice|  dice.iter().sum()).collect();
    if dice_sums.len() == 1 {
        return println!("{}", dice_sums[0]);
    }

    let mut total = 0;
    for (i, dice) in a.iter().enumerate() {
        let s = dice_sums.iter().enumerate().filter(|(k, dice)| *k != i).map(|(k, dice_sum)| dice_sum).product::<usize>() % Mod;
        for j in dice {
            total = (total + j * s % Mod) % Mod;
        }
    }

    println!("{}", total);
}
