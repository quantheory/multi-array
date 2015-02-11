//! Type-level natural numbers used to parameterize the array types.
//!
//! The numbers provided are `N0`, `N1`, `N2`, ... `N32`. We only go up to 32
//! because those are the array sizes for which important traits (e.g. `Debug`
//! and `Clone`) are implemented.

use core::fmt::Debug;
use core::marker::Copy;

/// Trait for type-level natural numbers.
pub trait Nat {
    /// A `usize` array of the correct length (e.g. `[usize; 0]` for `N0`).
    ///
    /// We only provide an array type for `usize` because:
    ///  - That's all that's needed for this library.
    ///  - Without higher-kinded types, we would need a bit more complexity to
    ///    connect a generic array `[T; D]` to the type-level number `ND`.
    type USizeArray: Copy+Debug;
    /// The value that corresponds to this number.
    fn value() -> usize;
}

/// Trait for type-level positive numbers.
///
/// The `Suc` type below allows one to increment by one. This type has the
/// predecessor type as an associated type, i.e. it is used to decrement by one.
pub trait PosNat: Nat {
    /// Predecessor of the Self type, e.g. `N0` is `<N1 as PosNat>::Pre`.
    type Pre: Nat;
}

/// Type-level natural number representing `0`.
#[derive(Copy, Debug)]
pub struct N0;
/// Type representing the successor of the wrapped type, e.g. `N1` is `Suc<N0>`.
#[derive(Copy, Debug)]
pub struct Suc<T: Nat>;

impl Nat for N0 {
    type USizeArray = [usize; 0];
    #[inline]
    fn value() -> usize { 0 }
}

macro_rules! suc_nat_impl {
    ($(($pre: ty, $cur: ident, $n: expr)),+) => ( $(
        /// Type-level natural number representing `$n`.
        pub type $cur = Suc<$pre>;
        impl Nat for $cur {
            type USizeArray = [usize; $n];
            #[inline]
            fn value() -> usize { $n }
        }
        )*
    );
}

suc_nat_impl!(
 ( N0,  N1,  1), ( N1,  N2,  2), ( N2,  N3,  3), ( N3,  N4,  4), ( N4,  N5,  5),
 ( N5,  N6,  6), ( N6,  N7,  7), ( N7,  N8,  8), ( N8,  N9,  9), ( N9, N10, 10),
 (N10, N11, 11), (N11, N12, 12), (N12, N13, 13), (N13, N14, 14), (N14, N15, 15),
 (N15, N16, 16), (N16, N17, 17), (N17, N18, 18), (N18, N19, 19), (N19, N20, 20),
 (N20, N21, 21), (N21, N22, 22), (N22, N23, 23), (N23, N24, 24), (N24, N25, 25),
 (N25, N26, 26), (N26, N27, 27), (N27, N28, 28), (N28, N29, 29), (N29, N30, 30),
 (N30, N31, 31), (N31, N32, 32)
);

impl<T: Nat> PosNat for Suc<T> where Suc<T>: Nat {
    type Pre = T;
}


/// Run four tests for each named Nat (three for N0 and N32):
///  1. The output of value() is correct.
///  2. A variable of the appropriate size can be assigned to USizeArray.
///  3. The predecessor has the correct value() output.
///  4. The successor has the correct value() output.
#[cfg(test)]
mod tests {
    use super::{N0, N1, N31, N32, Nat, PosNat, Suc};

    #[test]
    fn n0_value_is_0() {
        assert_eq!(<N0 as Nat>::value(), 0);
    }
    #[test]
    fn n0_array_is_assignable() {
        let _: <N0 as Nat>::USizeArray = [0us; 0];
    }
    #[test]
    fn n0_suc_value_is_1() {
        assert_eq!(<Suc<N0> as Nat>::value(), 1);
    }

    #[test]
    fn n1_value_is_1() {
        assert_eq!(<N1 as Nat>::value(), 1);
    }
    #[test]
    fn n1_array_is_assignable() {
        let _: <N1 as Nat>::USizeArray = [0us; 1];
    }
    #[test]
    fn n1_pre_value_is_0() {
        assert_eq!(<<N1 as PosNat>::Pre as Nat>::value(), 0);
    }
    #[test]
    fn n1_suc_value_is_2() {
        assert_eq!(<Suc<N1> as Nat>::value(), 2);
    }

    #[test]
    fn n31_value_is_31() {
        assert_eq!(<N31 as Nat>::value(), 31);
    }
    #[test]
    fn n31_array_is_assignable() {
        let _: <N31 as Nat>::USizeArray = [0us; 31];
    }
    #[test]
    fn n31_pre_value_is_30() {
        assert_eq!(<<N31 as PosNat>::Pre as Nat>::value(), 30);
    }
    #[test]
    fn n31_suc_value_is_32() {
        assert_eq!(<Suc<N31> as Nat>::value(), 32);
    }

    #[test]
    fn n32_value_is_32() {
        assert_eq!(<N32 as Nat>::value(), 32);
    }
    #[test]
    fn n32_array_is_assignable() {
        let _: <N32 as Nat>::USizeArray = [0us; 32];
    }
    #[test]
    fn n32_pre_value_is_30() {
        assert_eq!(<<N32 as PosNat>::Pre as Nat>::value(), 31);
    }
}
