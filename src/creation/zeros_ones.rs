use crate::core::tensor::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;

impl<T: Num + Clone + Copy> Tensor<T> {
    pub fn zero(shapes: Vec<usize>) -> Self {
        /*
            Тензор заполненный нулями
         */
        let values: Vec<T> = vec![T::zero(); shapes.iter().product()];
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
    pub fn ones(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vec![T::one(); shapes.iter().product()];
        let strides: Vec<usize> = compute_strides(&shapes);
        Self {values, shapes, strides }
    }
}