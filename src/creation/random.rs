use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use crate::core::tensor::Tensor;
use num_traits::Num;
use crate::core::strides::compute_strides;
use rand::distributions::Uniform;

/*
    Можно генерировать случайные числа для любого типа T через random() 😎
*/

impl<T: Num + Clone + Copy + rand::distributions::uniform::SampleUniform> Tensor<T> {
    pub fn random(shapes: Vec<usize>, range: (T, T)) -> Self {
        let mut rng: ThreadRng = thread_rng();
        let uniform: Uniform<T> = Uniform::new(range.0, range.1);
        let values: Vec<T> = (0..shapes.iter().product()).map(|_| uniform.sample(&mut rng)).collect();
        let strides: Vec<usize> = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}
/*
    дублирование кода ибо я не смог понять, как сделать обобщенный T удовлетворяющий условиям библиотеки для генерации случайных чисел
    Мб позже вернусь, но пока реализация генерации с нормальным распределением готова только для f32 и f64.
*/
impl Tensor<f64> {
    pub fn random_normal(shapes: Vec<usize>, mean: f64, std_dev: f64) -> Self {
        let mut rng = thread_rng();
        // Нормальное распределение требует, чтобы std_dev > 0
        let normal = Normal::new(mean, std_dev)
            .expect("std_dev должен быть положительным");
        let n = shapes.iter().product();
        let values: Vec<f64> = (0..n).map(|_| normal.sample(&mut rng)).collect();
        let strides = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}
impl Tensor<f32> {
    pub fn random_normal(shapes: Vec<usize>, mean: f32, std_dev: f32) -> Self {
        let mut rng = thread_rng();
        // Нормальное распределение требует, чтобы std_dev > 0
        let normal = Normal::new(mean, std_dev)
            .expect("std_dev должен быть положительным");
        let n = shapes.iter().product();
        let values: Vec<f32> = (0..n).map(|_| normal.sample(&mut rng)).collect();
        let strides = compute_strides(&shapes);
        Self { values, shapes, strides }
    }
}