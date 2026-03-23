use crate::core::tensor::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;

impl <T: Num + Clone + Copy> Tensor<T> {
    pub fn eye_2_d(size: usize) -> Self {
        let mut values: Vec<T>  = vec![T::zero(); size * size];
        let shapes: Vec<usize> = vec![size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        let _diagonal_step = strides[0] + strides[1];
        for k in 0..size {
            values[k * _diagonal_step] = T::one();
        }
        Self { values, shapes, strides }
    }
}