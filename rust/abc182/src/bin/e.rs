#![allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;
use std::io::Write;
use std::ops::Bound::*;
use itertools::*;

enum Status {
    None,
    Block,
    LightSource,
    Light,
}

fn main() {
    input! {
        (h, w, n, m): (usize, usize, usize, usize),
        ab: [(usize, usize); n],
        cd: [(usize, usize); m],
    }

    let mut field = vec![vec![&Status::None; w]; h];
    for (a, b) in ab {
        field[a-1][b-1] = &Status::LightSource;
    }
    for (c, d) in cd {
        field[c-1][d-1] = &Status::Block;
    }

    for y in 0..h {
        let mut is_light = false;
        for x in 0..w {
            match field[y][x] {
                Status::None => if is_light { field[y][x] = &Status::Light },
                Status::Block => is_light = false,
                Status::LightSource => is_light = true,
                 _ => (),
            }
        }
        is_light = false;
        for x in (0..w).rev() {
            match field[y][x] {
                Status::None => if is_light { field[y][x] = &Status::Light },
                Status::Block => is_light = false,
                Status::LightSource => is_light = true,
                 _ => (),
            }
        }
    }

    for x in 0..w {
        let mut is_light = false;
        for y in 0..h {
            match field[y][x] {
                Status::None => if is_light { field[y][x] = &Status::Light },
                Status::Block => is_light = false,
                Status::LightSource => is_light = true,
                 _ => (),
            }
        }
        is_light = false;
        for y in (0..h).rev() {
            match field[y][x] {
                Status::None => if is_light { field[y][x] = &Status::Light },
                Status::Block => is_light = false,
                Status::LightSource => is_light = true,
                 _ => (),
            }
        }
    }

    println!("{}", n + iproduct!(0..h, 0..w).filter(|pos| {
        match *field[pos.0][pos.1] {
            Status::Light => true,
            _ => false
        }
    }).count());
}

