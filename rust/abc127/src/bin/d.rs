#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use itertools_num::ItertoolsNum;
use itertools::__std_iter::once;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() { 
    input! {
        n: usize, m: usize,
        mut a: [usize; n],
        mut bc: [(usize, usize); m],
    }

    a.sort();
    bc.sort_by_key(|x| x.1);
    bc.reverse();

    let mut bci = 0;
    for i in 0..n {
        if a[i] > bc[bci].1 {
            break
        }
        a[i] = bc[bci].1;
        bc[bci].0 -= 1;
        if bc[bci].0 == 0 {
            bci += 1;
        }
        if bci >= m {
            break
        }
    }

    println!("{}", a.iter().sum::<usize>());
}
