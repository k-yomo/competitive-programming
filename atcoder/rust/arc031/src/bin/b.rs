#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;

use itertools::*;
use proconio::*;
use proconio::marker::*;
use superslice::*;

fn main() {
    input! {
        mut graph: [Chars; 10],
    }

    for (y, x) in iproduct!(0..10, 0..10) {
        if graph[y][x] == 'o' {
            continue;
        }
        graph[y][x] = 'o';

        let mut reached = vec![vec![false; 10]; 10];
        let mut island_count = 0;
        for (y, x) in iproduct!(0..10, 0..10) {
            if graph[y][x] == 'x' || reached[y][x] {
                continue;
            }
            island_count += 1;
            if island_count > 1 {
                break;
            }
            dfs(&graph, &mut reached, y, x);
        }
        if island_count == 1 {
            println!("YES");
            return;
        }
        graph[y][x] = 'x';
    }

    println!("NO");
}

fn dfs(graph: &Vec<Vec<char>>, reached: &mut Vec<Vec<bool>>, y: usize, x: usize) {
    if reached[y][x] {
        return;
    }

    reached[y][x] = true;
    if graph[y][x] == 'x' {
        return;
    }
    if y < 9 { dfs(graph, reached, y + 1, x); }
    if y > 0 { dfs(graph, reached, y - 1, x); }
    if x < 9 { dfs(graph, reached, y, x + 1); }
    if x > 0 { dfs(graph, reached, y, x - 1); }
}