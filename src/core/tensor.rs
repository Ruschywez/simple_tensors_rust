use num_traits::Num;
use std::any::TypeId;
use crate::core::strides::compute_strides;
// вот оно само величие его тензор

#[derive(Debug, PartialEq)]
pub struct Tensor<T: Num> {
    pub values: Vec<T>,
    pub shapes: Vec<usize>,
    pub strides: Vec<usize>,
}

impl<T: Num + Clone> Tensor<T> {
    pub fn zero(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vec![T::zero(); shapes.iter().product()];
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
    pub fn is_same_types<L: 'static, U: 'static>(_: &Vec<L>, _: &Vec<U>) -> bool {
        TypeId::of::<L>() == TypeId::of::<U>()
    }
    pub fn shapes(&self) -> Vec<usize> {
        self.shapes.clone()
    }
    pub fn strides(&self) -> Vec<usize> {
        self.strides.clone()
    }
    pub fn get(&self, indices: &[usize]) -> &T {
        // получение элемента по многомерному индексу
        let mut offset = 0;
        for (i, &dim_idm) in indices.iter().enumerate() {
            offset += dim_idm * self.strides[i];
        }
        &self.values[offset]
    }
}