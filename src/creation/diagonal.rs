use crate::core::tensor::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;

/*
    Методы для генерации диагональных матриц и кубов
    диагонали единицы, а всё остальное - нули.

    Методы с вариативными заполнителями ниже.
*/
impl <T: Num + Clone + Copy> Tensor<T> {
    pub fn eye_2_d(size: usize) -> Self {
        /*
            /// Главная квадратная диагональ
         */
        let mut values: Vec<T>  = vec![T::zero(); size * size]; // 2D
        let shapes: Vec<usize> = vec![size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        let _diagonal_step = strides[0] + strides[1];
        for k in 0..size {
            values[k * _diagonal_step] = T::one();
        }
        Self { values, shapes, strides }
    }
    pub fn secondary_diagonal_2_d(size: usize) -> Self {
        /*
            /// Побочная квадратная диагональ
         */
        let mut values: Vec<T>  = vec![T::zero(); size * size]; // 2D
        let shapes: Vec<usize> = vec![size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        for i in 0..size {
            values[i * strides[0] + (size - 1 - i) * strides[1]] = T::one();
        }
        Self { values, shapes, strides }
    }
    pub fn eye_3_d(size: usize) -> Self {
        /*
            /// Главная кубическая диагональ
         */
        let mut values: Vec<T>  = vec![T::zero(); size * size * size]; // 3D
        let shapes: Vec<usize> = vec![size, size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        // для глав. диагонали шаг - это сумма всех strides
        let _step: usize = strides[0] + strides[1] + strides[2];
        for i in 0..size {
            values[i * _step] = T::one();
        }
        Self { values, shapes, strides }
    }
    pub fn secondary_diagonal_3_d(size: usize) -> Self {
        /*
            /// Побочная кубическая диагональ
            (0, 0, size-1) -> (size-1, size-1, 0)
         */
        let mut values: Vec<T>  = vec![T::zero(); size * size * size]; // 3D
        let shapes: Vec<usize> = vec![size, size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        for i in 0..size {
            /*
                idx = i * N^2 + i * N + (N - 1 - i) * 1
             */
            values[i * strides[0] + i * strides[1] + (size - 1 - i) * strides[2]] = T::one();
        }
        Self { values, shapes, strides }
    }
}

impl <T: Num + Clone + Copy> Tensor<T> {
    pub fn main_diagonal_2_d_with_values(size: usize, diagonal: T, fill: T) -> Self {
        /*
            /// Главная квадратная диагональ
         */
        let mut values: Vec<T>  = vec![fill; size * size]; // 2D
        let shapes: Vec<usize> = vec![size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        let _diagonal_step = strides[0] + strides[1];
        for k in 0..size {
            values[k * _diagonal_step] = diagonal;
        }
        Self { values, shapes, strides }
    }
    pub fn secondary_diagonal_2_d_with_values(size: usize, diagonal: T, fill: T) -> Self {
        /*
            /// Побочная квадратная диагональ
         */
        let mut values: Vec<T>  = vec![fill; size * size]; // 2D
        let shapes: Vec<usize> = vec![size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        for i in 0..size {
            values[i * strides[0] + (size - 1 - i) * strides[1]] = diagonal;
        }
        Self { values, shapes, strides }
    }
    pub fn main_diagonal_3_d_with_values(size: usize, diagonal: T, fill: T) -> Self {
        /*
            /// Главная кубическая диагональ
         */
        let mut values: Vec<T>  = vec![fill; size * size * size]; // 3D
        let shapes: Vec<usize> = vec![size, size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        // для глав. диагонали шаг - это сумма всех strides
        let _step: usize = strides[0] + strides[1] + strides[2];
        for i in 0..size {
            values[i * _step] = diagonal;
        }
        Self { values, shapes, strides }
    }
    pub fn secondary_diagonal_3_d_with_values(size: usize, diagonal: T, fill: T) -> Self {
        /*
            /// Побочная кубическая диагональ
            (0, 0, size-1) -> (size-1, size-1, 0)
         */
        let mut values: Vec<T>  = vec![fill; size * size * size]; // 3D
        let shapes: Vec<usize> = vec![size, size, size];
        let strides: Vec<usize> = compute_strides(&shapes);
        for i in 0..size {
            /*
                idx = i * N^2 + i * N + (N - 1 - i) * 1
             */
            values[i * strides[0] + i * strides[1] + (size - 1 - i) * strides[2]] = diagonal;
        }
        Self { values, shapes, strides }
    }
}