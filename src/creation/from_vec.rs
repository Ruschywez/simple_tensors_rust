use crate::core::tensor::Tensor;
use crate::core::strides::compute_strides;
use num_traits::Num;

impl<T: Num + Clone + Copy> Tensor<T> {
    pub fn from_vec(vector: Vec<T>, shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vector;
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
    pub fn clone_from_vec(vector: &[T], shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vector.to_vec();
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}