#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use std::char;
 
fn main() { 
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h]
    }

    for (i, row) in s.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '#' {
                print!("#");
                continue
            }
            let mut bomb_count = 0;
            for surrounding in get_surroundings(h,w, i, j) {
                if s[surrounding.0][surrounding.1] == '#' {
                    bomb_count += 1;
                }
            }
            print!("{}", bomb_count);
        }
        println!()
    }

}

fn get_surroundings(h: usize, w: usize, y: usize, x: usize) -> Vec::<(usize, usize)> {
	let surrounding_diffs: Vec::<(i64, i64)> = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    let mut surroundings = vec![];
	for diff in surrounding_diffs.iter() {
		let a = y as i64 + diff.0;
        let b = x as i64 + diff.1;
        if a < 0 || b < 0 || a >= h as i64 || b >= w as i64 {
			continue
        }
        surroundings.push((a as usize, b as usize));
	}
	surroundings
}