#![allow(unused_imports, non_snake_case, unreachable_code)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut ans = std::usize::MAX;
    let mut seg = SegmentTree::from(vec![Ele(-1); n]);
    seg.set(0, Ele(0));
    for i in 1..(n - m) {
        let (u, _) = seg.partition(|_, x| x.0 >= (i - a[i - 1]) as isize);
        ans = std::cmp::min(ans, u);
        seg.set(u, Ele(i as isize));
    }
    println!("{}", ans);
}

#[derive(Clone, Debug, PartialEq)]
struct Ele(isize);

impl Add for Ele {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(std::cmp::min(self.0, rhs.0))
    }
}

impl Associative for Ele {}

impl Zero for Ele {
    fn zero() -> Self {
        Self(std::isize::MAX)
    }
    fn is_zero(&self) -> bool {
        self.0 == std::isize::MAX
    }
}
// ------------ FenwickTree with generics start ------------

pub struct SegmentTree<T: Monoid> {
    n: usize,
    size: usize,
    node: Vec<T>,
}

impl<T: Monoid> SegmentTree<T> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        let node = vec![T::zero(); size * 2];
        SegmentTree { n, size, node }
    }

    pub fn set(&mut self, mut i: usize, x: T) {
        i += self.size;
        self.node[i] = x;
        self.fix(i);
    }

    fn fix(&mut self, mut i: usize) {
        while i > 0 {
            i >>= 1;
            self.node[i] = self.node[i << 1].clone() + self.node[(i << 1) + 1].clone();
        }
    }

    pub fn fold<R: RangeBounds<usize>>(&self, rng: R) -> T {
        let Range { start, end } = bounds_within(rng, self.size);
        let mut vl = T::zero();
        let mut vr = T::zero();
        let mut l = start + self.size;
        let mut r = end + self.size;
        while l < r {
            if l & 1 == 1 {
                vl = vl + self.node[l].clone();
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = self.node[r].clone() + vr;
            }
            l >>= 1;
            r >>= 1;
        }
        vl + vr
    }

    /// (j, t) => pred(j-1) = true, pred(j) = false
    pub fn partition(&self, pred: impl Fn(usize, &T) -> bool) -> (usize, T) {
        assert!(pred(0, &T::zero()), "need to be pred(0, T::zero())");
        if pred(self.n - 1, &self.node[1]) {
            return (self.n - 1, self.node[1].clone());
        }
        let mut j = 1;
        let mut current = T::zero();
        let mut idx = 0;
        let mut f = self.size;
        while j < self.size {
            j <<= 1;
            f >>= 1;
            let next = current.clone() + self.node[j].clone();
            if pred(idx + f - 1, &next) {
                current = next;
                j |= 1;
                idx += f;
            }
        }
        (idx, current)
    }
}

impl<T: Monoid> From<Vec<T>> for SegmentTree<T> {
    fn from(vec: Vec<T>) -> Self {
        let n = vec.len();
        let size = n.next_power_of_two();
        let mut node = vec![T::zero(); size << 1];
        for (i, e) in vec.iter().cloned().enumerate() {
            node[i + size] = e;
        }
        for i in (1..size).rev() {
            node[i] = node[i << 1].clone() + node[(i << 1) + 1].clone();
        }
        SegmentTree { n, size, node }
    }
}

impl<T: Monoid> Index<usize> for SegmentTree<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(
            i < self.size,
            "index out of range: length is {}, but given {}.",
            self.size,
            i
        );
        &self.node[i + self.size]
    }
}
// ------------ FenwickTree with generics end ------------

use std::marker::Sized;
use std::ops::*;

/// 元
pub trait Element: Sized + Clone + PartialEq {}
impl<T: Sized + Clone + PartialEq> Element for T {}

/// 結合性
pub trait Associative: Magma {}

/// マグマ
pub trait Magma: Element + Add<Output = Self> {}
impl<T: Element + Add<Output = Self>> Magma for T {}

/// 半群
pub trait SemiGroup: Magma + Associative {}
impl<T: Magma + Associative> SemiGroup for T {}

/// モノイド
pub trait Monoid: SemiGroup + Zero {}
impl<T: SemiGroup + Zero> Monoid for T {}

pub trait ComMonoid: Monoid + AddAssign {}
impl<T: Monoid + AddAssign> ComMonoid for T {}

/// 群
pub trait Group: Monoid + Neg<Output = Self> {}
impl<T: Monoid + Neg<Output = Self>> Group for T {}

pub trait ComGroup: Group + ComMonoid {}
impl<T: Group + ComMonoid> ComGroup for T {}

/// 半環
pub trait SemiRing: ComMonoid + Mul<Output = Self> + One {}
impl<T: ComMonoid + Mul<Output = Self> + One> SemiRing for T {}

/// 環
pub trait Ring: ComGroup + SemiRing {}
impl<T: ComGroup + SemiRing> Ring for T {}

pub trait ComRing: Ring + MulAssign {}
impl<T: Ring + MulAssign> ComRing for T {}

/// 体
pub trait Field: ComRing + Div<Output = Self> + DivAssign {}
impl<T: ComRing + Div<Output = Self> + DivAssign> Field for T {}

/// 加法単元
pub trait Zero: Element {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

/// 乗法単元
pub trait One: Element {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! impl_integer {
    ($($T:ty,)*) => {
        $(
            impl Associative for $T {}

            impl Zero for $T {
                fn zero() -> Self { 0 }
                fn is_zero(&self) -> bool { *self == 0 }
            }

            impl<'a> Zero for &'a $T {
                fn zero() -> Self { &0 }
                fn is_zero(&self) -> bool { *self == &0 }
            }

            impl One for $T {
                fn one() -> Self { 1 }
                fn is_one(&self) -> bool { *self == 1 }
            }

            impl<'a> One for &'a $T {
                fn one() -> Self { &1 }
                fn is_one(&self) -> bool { *self == &1 }
            }
        )*
    };
}

impl_integer! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Range, RangeBounds};

pub fn bounds_within<R: RangeBounds<usize>>(r: R, len: usize) -> Range<usize> {
    let e_ex = match r.end_bound() {
        Included(&e) => e + 1,
        Excluded(&e) => e,
        Unbounded => len,
    }
    .min(len);
    let s_in = match r.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    }
    .min(e_ex);
    s_in..e_ex
}
