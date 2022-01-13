#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use std::iter::FromIterator;

use itertools::*;
use itertools::__std_iter::once;
use itertools_num::ItertoolsNum;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize; n],
    }

    let mut priority_queue = BinaryHeap::from_iter(p[..k].iter().map(|&i| Reverse(i)));

    println!("{}", priority_queue.peek().unwrap().0);
    for &num in p[k..].iter() {
        if num > priority_queue.peek().unwrap().0 {
            priority_queue.pop();
            priority_queue.push(Reverse(num));
        }
        println!("{}", priority_queue.peek().unwrap().0);
    }
}
