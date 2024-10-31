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
        N:usize,W:usize,
        WV:[(usize, usize); N]
    }
    let mut dp = vec![vec![0; W + 10]; N + 10];
    for i in 1..N + 1 {
        let (w, v) = WV[i - 1];
        for j in 0..W + 1 {
            dp[i][j] = dp[i][j].max(dp[i - 1][j]);
            // if j > 0 {
            //     dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            // }
            if j >= w {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - w] + v)
            }
        }
    }
    let mut ans = 0;
    for i in 0..W + 1 {
        ans = ans.max(dp[N][i]);
    }
    println!("{}", ans);
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

