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
        A:[isize; N],
        Q:usize,
        LR:[(usize, usize); Q]
    }
    let mut accum = vec![0];
    for (i, &a) in A.iter().enumerate() {
        if a == 0 {
            accum.push(accum[i] - 1);
        } else {
            accum.push(accum[i] + 1);
        }
    }
    let mut ans = vec![];
    for &(l, r) in LR.iter() {
        let tmp = accum[r] - accum[l - 1];
        if tmp > 0 {
            ans.push("win");
        } else if tmp == 0 {
            ans.push("draw");
        } else {
            ans.push("lose");
        }
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

