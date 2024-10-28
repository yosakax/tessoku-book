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
        H:usize, W:usize,
        mut X:[[usize; W]; H],
        Q:usize,
        ABCD:[(Usize1, Usize1, Usize1, Usize1); Q]
    }
    for i in 0..H {
        for j in 0..W {
            if i == 0 && j == 0 {
                continue;
            }
            if j > 0 {
                X[i][j] += X[i][j - 1];
            }
            if i > 0 {
                X[i][j] += X[i - 1][j];
            }
            if i > 0 && j > 0 {
                X[i][j] -= X[i - 1][j - 1];
            }
        }
    }
    let mut ans = vec![];
    for &(a, b, c, d) in ABCD.iter() {
        let mut tmp = X[c][d];
        if a > 0 && b > 0 {
            tmp += X[a - 1][b - 1];
        }
        if a > 0 {
            tmp -= X[a - 1][d];
        }
        if b > 0 {
            tmp -= X[c][b - 1];
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

