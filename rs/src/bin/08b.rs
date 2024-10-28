#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
use ac_library::*;
use amplify::confinement::Collection;
use itertools::{CombinationsWithReplacement, Itertools};
use lazy_static::lazy_static;
use libm::{ceil, log};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_bigint::BigInt;
use num_integer::{div_ceil, Integer};
use proconio::marker::{Chars, Usize1};
use proconio::{input, source::line::LineSource};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::From;
use std::f64::consts::PI;
use std::hash::Hash;
use std::mem::swap;
use std::ops::Index;
use std::ops::Mul;
use std::{io::*, string};

fn solve() {
    #[rustfmt::skip]
    input! {
        N:usize,
        XY:[(usize, usize); N],
        Q:usize,
        ABCD:[(usize,usize,usize,usize); Q]
    }
    let mut mat = vec![vec![0; 1510]; 1510];
    for &(x, y) in XY.iter() {
        mat[x][y] += 1;
    }
    for i in 0..1510 {
        for j in 0..1510 {
            if i > 0 {
                mat[i][j] += mat[i - 1][j];
            }
            if j > 0 {
                mat[i][j] += mat[i][j - 1];
            }
            if i > 0 && j > 0 {
                mat[i][j] -= mat[i - 1][j - 1];
            }
        }
    }
    let mut ans = vec![];
    for &(a, b, c, d) in ABCD.iter() {
        let mut tmp = mat[c][d];
        if a > 0 {
            tmp -= mat[a - 1][d];
        }
        if b > 0 {
            tmp -= mat[c][b - 1];
        }
        if a > 0 && b > 0 {
            tmp += mat[a - 1][b - 1];
        }
        ans.push(tmp);
    }
    println!("{}", ans.iter().join("\n"));
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

