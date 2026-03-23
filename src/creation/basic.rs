use crate::core::tensor::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;

impl<T: Num + Clone + Copy> Tensor<T> {
    pub fn new(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = Vec::with_capacity(shapes.iter().product());
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
    pub fn fill(shapes: Vec<usize>, value: T) -> Self {
        let values: Vec<T> = vec![value; shapes.iter().product()];
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}