//! This crate provides a set of multidimensional arrays, which are intended to
//! be efficient for use in numerically intensive code.
//!
//! The types here have their dimension specified by a type with the `PosNat`
//! trait, which is one of `N1`, `N2`, ..., `N32`.

#![feature(alloc)]
#![feature(box_syntax)]
#![feature(core)]
#![feature(no_std)]
#![feature(unsafe_destructor)]
#![no_std]

#![warn(missing_copy_implementations, missing_debug_implementations)]
#![warn(missing_docs, unused, variant_size_differences)]

extern crate alloc;
#[macro_use]
extern crate core;
#[cfg(test)]
extern crate std;

pub mod typenat;
pub mod array;

pub use typenat::{ N0,  N1,  N2,  N3,  N4,  N5,  N6,  N7,  N8,  N9,
                  N10, N11, N12, N13, N14, N15, N16, N17, N18, N19,
                  N20, N21, N22, N23, N24, N25, N26, N27, N28, N29,
                  N30, N31, N32};
pub use array::{MDArrayBuf, MDArrayView};
