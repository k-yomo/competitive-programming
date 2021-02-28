#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;
use itertools_num::ItertoolsNum;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    }

    let mut xor_acc = vec![a[0];n+1];
    for (i, num) in a[1..].iter().enumerate() {
        xor_acc[i+1] = xor_acc[i] ^ num;
    }

    for (t, x, y) in txy {
        if t == 1 {
            a[x-1] = a[x-1] ^ y;
        } else {
            println!("{}", xor_acc[y-1] ^ xor_acc[x-1]);
        }
    }
}
