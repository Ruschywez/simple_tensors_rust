use std::ops::Sub;
use crate::tensor::Tensor;
use num_traits::Num;

/*
    Модуль разницы.
    На момент 19.03.2026 здесь реализованы только 4 трейта:
    t  - t
    t  - &t
    &t - t
    &t - &t
*/

// Разница
impl<T: Num + Clone> Sub for Tensor<T> {
    type Output = Tensor<T>;  // Было TEnsor<T>
    
    fn sub(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match!");
        let values = self.values.iter()
            .zip(other.values.iter())  // Было valies
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Tensor { values, shapes: self.shapes, strides: self.strides }
    }
}

impl<T: Num + Clone> Sub for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn sub(self, other: &Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();  // Было colect
        Tensor { values, shapes: self.shapes.clone(), strides: self.strides.clone() }
    }
}

impl<T: Num + Clone> Sub<&Tensor<T>> for Tensor<T> {
    type Output = Tensor<T>;
    
    fn sub(self, other: &Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Tensor { values, shapes: self.shapes, strides: self.strides }
    }
}

impl<T: Num + Clone> Sub<Tensor<T>> for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn sub(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");  // Было "nust"
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Tensor { values, shapes: self.shapes.clone(), strides: self.strides.clone() }
    }
}