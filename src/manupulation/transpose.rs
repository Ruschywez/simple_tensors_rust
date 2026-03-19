use crate::tensor::Tensor;

impl<T: Num + Clone> Tensor<T> {
    pub fn transpose_lazy(&mut self){
        // ### Lazy Transpose Algorithm
        // ленивое транспонирование всего тензора.
        // не перемешивает объекты, а разворачиавет оси shapes и шаги strides
        self.shapes.reverse();
        self.strides.reverse();
    }
    pub fn transpose_physical_N_shapes(&mut self) {
        // Полное транспонирование
        // Подходит для тензоров с большим количеством измерений
        // для тензоров 2D и 3D стоит использовать специальные методы
        // дорого по памяти
        if self.shapes.len() <= 1 {
            return;
        }
        // клонирование характеристик
        let original_shapes: Vec<usize> = self.shapes.clone();
        let orifinal_strides: Vec<usize> = self.strides.clone();
        // разворот shapes и strides
        let new_shapes: Vec<usize> = original_shapes.iter().rev().cloned().collect();
        let mut new_strides: Vec<usize> = Vec::with_capacity(new_shapes.len());
        let mut stride: usize = 1;
        for &dim in new_shapes.iter().rev() {
            new_strides.push(stride);
            stride *= dim;
        }
        new_strides.reverse(); // ёу разворот еее
        let mut index_map: Vec<usize> = vec![0; self.values.len()];
        for old_linear_idx in 0..self.values.len() {
            let mut remaining_idx: usize = old_linear_idx;
            let mut coords: Vec<usize> = Vec::with_capacity(original_shapes.len());
            for &stride in &orifinal_strides {
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
            new_values[new_idx] = self.values[old_idx];
        }
        self.values = new_values;
        self.shapes = new_shapes;
        self.strides = new_strides;
    }
}