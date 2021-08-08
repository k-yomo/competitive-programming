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

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n-1],
    }

    let mut path = HashMap::new();
    for (a, b) in ab.iter() {
        path.entry(a - 1)
            .or_insert(BinaryHeap::new())
            .push(Rev(b - 1));
        path.entry(b - 1)
            .or_insert(BinaryHeap::new())
            .push(Rev(a - 1));
    }

    let mut visited = vec![false; n];
    let mut visited_path = vec![];
    let mut cur_city = 0;
    loop {
        visited[cur_city] = true;
        let cities = path.entry(cur_city).or_default();
        let mut is_back = false;
        let mut next_city = cur_city;
        while visited[next_city] {
            if cities.len() == 0 {
                if visited_path.len() == 0 {
                    print!("1");
                    return println!("");
                }
                next_city = visited_path.pop().unwrap();
                is_back = true;
                break;
            } else {
                let Rev(city) = cities.pop().unwrap();
                next_city = city;
            }
        }

        print!("{} ", cur_city + 1);
        if !is_back {
            visited_path.push(cur_city);
        }
        cur_city = next_city;
    }
}
