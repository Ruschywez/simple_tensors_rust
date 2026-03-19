use crate::Tensor;
use num_traits::Num;

impl<T: Num + Clone> Tensor<T> {
    pub fn transpose_lazy(&mut self){
        // ### Lazy Transpose Algorithm
        // ленивое транспонирование всего тензора.
        // не перемешивает объекты, а разворачивает оси shapes и шаги strides
        self.shapes.reverse();
        self.strides.reverse();
    }
    pub fn transpose_physical_n_shapes(&mut self) {
        /*
            Пока самая моя нелюбимая и некрасивая функция!
         */
        if self.shapes.len() <= 1 as usize {
            return;
        }
        let original_shapes: Vec<usize> = self.shapes.clone();
        let original_strides: Vec<usize> = self.strides.clone();
        let new_shapes: Vec<usize> = original_shapes.iter().rev().cloned().collect();
        let mut new_strides: Vec<usize> = Vec::with_capacity(new_shapes.len());
        let mut stride: usize = 1;
        for &dim in new_shapes.iter().rev() {
            new_strides.push(stride);
            stride *= dim;
        }
        new_strides.reverse();
        let mut index_map: Vec<usize> = vec![0; self.values.len()];
        for old_linear_idx in 0..self.values.len() {
            let mut remaining_idx: usize = old_linear_idx;
            let mut coords: Vec<usize> = Vec::with_capacity(original_shapes.len());
            for &stride in &original_strides {
                coords.push(remaining_idx / stride);
                remaining_idx %= stride;
            }
            coords.reverse();
            let mut new_linear_idx: usize = 0;
            for (j, (&coord, &stride)) in coords.iter().zip(&new_strides).enumerate() {
                new_linear_idx += coord * stride;
            }
            index_map[old_linear_idx] = new_linear_idx;
        }
        let mut new_values: Vec<T> = vec![T::zero(); self.values.len()];
        for (old_idx, &new_idx) in index_map.iter().enumerate() {
            new_values[new_idx] = self.values[old_idx].clone();
        }
        self.values = new_values;
        self.shapes = new_shapes;
        self.strides = new_strides;
    }
}