use std::ops::Add;
use crate::tensor::Tensor;
use num_traits::Num;

/*
    Модуль сложения.
    На момент 19.03.2026 здесь реализованы только 4 трейта:
    t  + t
    t  + &t
    &t + t
    &t + &t
*/


// СЛОЖЕНИЕ
impl<T: Num + Clone> Add for Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match!");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes, strides: self.strides }
    }
}

impl<T: Num + Clone> Add for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: &Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes.clone(), strides: self.strides.clone() }
    }
}

impl<T: Num + Clone> Add<&Tensor<T>> for Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: &Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes, strides: self.strides }
    }
}

impl<T: Num + Clone> Add<Tensor<T>> for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes.clone(), strides: self.strides.clone() }
    }
}