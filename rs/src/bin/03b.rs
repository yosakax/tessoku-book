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
        A:[usize; N]
    }
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                if A[i] + A[j] + A[k] == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}

