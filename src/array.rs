//! The multidimensional array types themselves.

use alloc::boxed::Box;

use typenat::{Nat, PosNat};

/// Trait representing a multidimensional array view (regardless of whether the
/// data is actually in an array).
pub trait MDArrayView<T, D: PosNat> {
    /// Type of an index into the array.
    ///
    /// This should never actually be changed from the default.
    type Ix = <D as Nat>::USizeArray;
    /// Shape of the array. For instance, a 4x5 array has shape `[4, 5]`.
    fn shape(&self) -> Self::Ix;
}

/// A multidimensional array that owns its data.
#[derive(Debug)]
pub struct MDArrayBuf<T, D: PosNat> {
    inner: Box<[T]>,
    shape: <D as Nat>::USizeArray,
}

impl<T, D: PosNat> MDArrayBuf<T, D> {
    /// Construct a MDArrayBuf from a boxed slice and a array specifying the
    /// shape.
    pub fn from_boxed_slice(boxed: Box<[T]>, shape: <D as Nat>::USizeArray)
                        -> Self {
        MDArrayBuf{inner: boxed, shape: shape}
    }
}

impl<T, D: PosNat> MDArrayView<T, D> for MDArrayBuf<T, D> {
    type Ix = <D as Nat>::USizeArray;
    fn shape(&self) -> <Self as MDArrayView<T, D>>::Ix { self.shape }
}

#[cfg(test)]
mod tests {
    use typenat::{N1, N2, N3};
    use super::{MDArrayBuf, MDArrayView};
    #[test]
    fn boxed_slice_constructs_correct_shape_1d() {
        let boxed_nums = box [1, 2, 3, 4];
        let array = MDArrayBuf::<i32, N1>::from_boxed_slice(boxed_nums, [4]);
        assert_eq!(array.shape(), [4us]);
    }
    #[test]
    fn boxed_slice_constructs_correct_shape_2d() {
        let boxed_nums = box [1, 2, 3, 4];
        let array = MDArrayBuf::<i32, N2>::from_boxed_slice(boxed_nums, [2, 2]);
        assert_eq!(array.shape(), [2us, 2]);
    }
    #[test]
    fn boxed_slice_constructs_correct_shape_3d() {
        let boxed_nums = box [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let array = MDArrayBuf::<i32, N3>::from_boxed_slice(boxed_nums,
                                                            [2, 3, 2]);
        assert_eq!(array.shape(), [2us, 3, 2]);
    }
}
