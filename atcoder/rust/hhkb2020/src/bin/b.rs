#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
 
fn main() { 
    input! {
        h: usize,
        w: usize,
        graph: [Chars; h],
    }

    let mut count = 0;
    for (i, row) in graph.iter().enumerate() {
        for (j, &row) in row.iter().enumerate() {
            if row != '.' {
                continue
            }
            if i < h-1 && graph[i+1][j] == '.' {
                count += 1;
            }
            if j < w-1 && graph[i][j+1] == '.' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
