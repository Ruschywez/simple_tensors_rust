use crate::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;

impl<T: Num + Clone> Tensor<T> {
    pub fn zero(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vec![T::zero(); shapes.iter().product()];
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}