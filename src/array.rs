//! The multidimensional array types themselves.
use alloc::boxed::Box;
use core::ops::{Index, IndexMut};

use typenat::{Nat, PosNat, USIndex};

/// Trait representing a multidimensional array view (regardless of whether the
/// data is actually in an array).
pub trait MDArrayView<T, D: PosNat>:
              Index<<D as Nat>::IxArray>+IndexMut<<D as Nat>::IxArray> {
    /// Shape of the array. For instance, a 4x5 array has shape `[4, 5]`.
    fn shape(&self) -> <D as Nat>::IxArray;
}

/// A multidimensional array that owns its data.
#[derive(Debug)]
pub struct MDArrayBuf<T, D: PosNat> {
    inner: Box<[T]>,
    shape: <D as Nat>::IxArray,
}

impl<T, D: PosNat> MDArrayBuf<T, D> {
    /// Construct a MDArrayBuf from a boxed slice and a array specifying the
    /// shape.
    #[inline]
    pub fn from_boxed_slice(boxed: Box<[T]>, shape: <D as Nat>::IxArray)
                        -> Self {
        MDArrayBuf{inner: boxed, shape: shape}
    }
}

impl<T, D: PosNat> Index<<D as Nat>::IxArray> for MDArrayBuf<T, D> {
    type Output = T;
    #[inline]
    fn index<'a>(&'a self, index: &<D as Nat>::IxArray) -> &'a T {
        // These unsafe blocks are OK because we know the size of the array
        // representing IxArray must be right at compile time.
        let mut pos = *unsafe{ index.us_index_unchecked(0) };
        for dim in 1..(<D as Nat>::value()) {
            pos *= *unsafe{ self.shape.us_index_unchecked(dim) };
            pos += *unsafe{ index.us_index_unchecked(dim) };
        }
        &self.inner[pos]
    }
}

impl<T, D: PosNat> IndexMut<<D as Nat>::IxArray> for MDArrayBuf<T, D> {
    #[inline]
    fn index_mut<'a>(&'a mut self, index: &<D as Nat>::IxArray) -> &'a mut T {
        // These unsafe blocks are OK because we know the size of the array
        // representing IxArray must be right at compile time.
        let mut pos = *unsafe{ index.us_index_unchecked(0) };
        for dim in 1..(<D as Nat>::value()) {
            pos *= *unsafe{ self.shape.us_index_unchecked(dim) };
            pos += *unsafe{ index.us_index_unchecked(dim) };
        }
        &mut self.inner[pos]
    }
}

impl<T, D: PosNat> MDArrayView<T, D> for MDArrayBuf<T, D> {
    #[inline]
    fn shape(&self) -> <D as Nat>::IxArray { self.shape }
}

#[cfg(test)]
mod tests {
    use collections::slice::SliceExt;
    use typenat::{N1, N2, N3};
    use super::{MDArrayBuf, MDArrayView};
    #[test]
    fn boxed_slice_constructs_correct_shape_1d() {
        let boxed_nums = box [1, 2, 3, 4];
        let array = MDArrayBuf::<i32, N1>::from_boxed_slice(boxed_nums, [4]);
        assert_eq!(array.shape(), [4us]);
    }
    #[test]
    fn boxed_slice_constructs_correct_shape_3d() {
        let boxed_nums = box [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let array = MDArrayBuf::<i32, N3>::from_boxed_slice(boxed_nums,
                                                            [2, 3, 2]);
        assert_eq!(array.shape(), [2us, 3, 2]);
    }
    #[test]
    fn index_retrieves_elements_in_row_major_order() {
        let data = [1i32, 2, 3, 4, 5, 6];
        let boxed_nums = box [1, 2, 3, 4, 5, 6];
        let array = MDArrayBuf::<_, N2>::from_boxed_slice(boxed_nums, [2, 3]);
        let mut pos = 0;
        for i in 0..2 {
            for j in 0..3 {
                assert_eq!(array[[i, j]], data[pos]);
                pos += 1;
            }
        }
    }
    #[test]
    fn index_mut_sets_elements_in_row_major_order() {
        let data = [1i32, 2, 3, 4, 5, 6];
        let boxed_nums = box [0; 6];
        let mut array =
            MDArrayBuf::<_, N2>::from_boxed_slice(boxed_nums, [2, 3]);
        let mut pos = 0;
        for i in 0..2 {
            for j in 0..3 {
                array[[i, j]] = data[pos];
                pos += 1;
            }
        }
        assert_eq!(array.inner.into_vec(), (&data).to_vec())
    }
}
