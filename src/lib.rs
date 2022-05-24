mod tmp;

use itertools::Itertools;
use num_integer::Integer;
use num_traits::Zero;
use proconio::input;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Range;

// Todo: 要素の数を数える [1, 1, 2, 2, 2, 3] => {1: 2, 2: 3, 3: 1}
// Todo: テストを書く
// Todo: Powerset(全探索用)
// Todo: 入力用マクロ
fn main() {
    input!{

    };
}



// フォーマット指定子なしでフォーマットを行う
#[macro_export]
#[allow(unused_macros)]
macro_rules! mformat {
    () => {""};
    (@replace $_t:tt, $sub:expr) => {$sub};
    ($x:expr $(,$y:expr)*)=> {
        format!(concat!("{}"$(, $crate::mformat!(@replace $y, " {}"))*), $x $(, $y)*)
    };
}
// フォーマット指定子なしで出力を行う
#[macro_export]
#[allow(unused_macros)]
macro_rules! mprint {
    () => {};
    ($x:expr $(,$y:expr)*)=> {print!("{}", $crate::mformat!($x $(, $y)*))}
}

// フォーマット指定子なしで出力を行う(改行あり)
#[macro_export]
#[allow(unused_macros)]
macro_rules! mprintln {
    () => {};
    ($x:expr $(,$y:expr)*)=> {println!("{}", $crate::mformat!($x $(, $y)*))}
}

#[allow(dead_code)]
fn print_yes_no(y: bool) {
    if y {
        println!("Yes")
    } else {
        println!("No")
    }
}

pub trait MyIterTools: Iterator
where
    Self: Sized,
{
    fn print(self)
    where
        Self::Item: Display,
    {
        print!("{}", self.format(" "));
    }
    fn println(self)
    where
        Self::Item: Display,
    {
        println!("{}", self.format(" "));
    }
    fn collect_set(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.collect()
    }
    fn collect_btree_set(self) -> BTreeSet<Self::Item>
    where
        Self::Item: Ord,
    {
        self.collect()
    }
    fn counts(self) -> HashMap<Self::Item, usize>
    where
        Self::Item: Eq + Hash,
    {
        let mut result = HashMap::new();
        self.for_each(|v| {
            *result.entry(v).or_insert(0) += 1;
        });
        result
    }
    fn counts_btree(self) -> BTreeMap<Self::Item, usize>
    where
        Self::Item: Ord,
    {
        let mut result = BTreeMap::new();
        self.for_each(|v| {
            *result.entry(v).or_insert(0) += 1;
        });
        result
    }
}

impl<T> MyIterTools for T where T: Iterator {}

pub trait Two
where
    Self: Sized + Eq,
{
    fn two() -> Self;
    fn is_two(&self) -> bool {
        self.eq(&Self::two())
    }
}

pub trait Three
where
    Self: Sized + Eq,
{
    fn three() -> Self;
    fn is_three(&self) -> bool {
        self.eq(&Self::three())
    }
}

pub trait Ten
where
    Self: Sized + Eq,
{
    fn ten() -> Self;
    fn is_ten(&self) -> bool {
        self.eq(&Self::ten())
    }
}

pub trait Abs
where
    Self: Sized,
{
    fn abs(&self) -> Self;
}

pub trait IntImpl
where
    Self:
        Copy + Integer + Two + Three + Ten + Zero + Abs + std::ops::AddAssign + std::ops::DivAssign,
{
    fn is_prime(&self) -> bool {
        if self <= &(Self::one()) {
            return false;
        }
        if self.is_two() {
            return true;
        }
        if self.is_multiple_of(&(Self::two())) {
            return false;
        }
        let mut i = Self::three();
        while &(i * i) <= self {
            if self.is_multiple_of(&i) {
                return false;
            }
            i += Self::two();
        }
        true
    }
    fn range(&self, start: Self) -> Range<Self> {
        start..*self + start
    }
    fn range0(&self) -> Range<Self> {
        self.range(Self::zero())
    }
    fn range1(&self) -> Range<Self> {
        self.range(Self::one())
    }
    fn digits(&self, n: Self) -> Vec<Self> {
        if self.is_zero() {
            return vec![Self::zero()];
        }
        let mut result = Vec::new();
        let mut x = self.abs();
        while x > Self::zero() {
            result.push(x % n);
            x /= n;
        }
        result.reverse();
        result
    }
    fn digits10(&self) -> Vec<Self> {
        self.digits(Self::ten())
    }
}

#[macro_export]
macro_rules! int_impl {
    ($t:ty) => {
        impl Two for $t {
            fn two() -> $t {
                2
            }
        }
        impl Three for $t {
            fn three() -> $t {
                3
            }
        }
        impl Ten for $t {
            fn ten() -> $t {
                10
            }
        }
        impl IntImpl for $t {}
    };
}

#[macro_export]
macro_rules! iint_impl {
    ($t:ty) => {
        impl Abs for $t {
            fn abs(&self) -> $t {
                (*self as $t).abs()
            }
        }
        $crate::int_impl!($t);
    };
}

#[macro_export]
macro_rules! uint_impl {
    ($t:ty) => {
        impl Abs for $t {
            fn abs(&self) -> $t {
                *self
            }
        }
        $crate::int_impl!($t);
    };
}

iint_impl!(i8);
iint_impl!(i16);
iint_impl!(i32);
iint_impl!(i64);
iint_impl!(i128);
iint_impl!(isize);
uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(u128);
uint_impl!(usize);
