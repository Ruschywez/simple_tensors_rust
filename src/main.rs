use num_traits::Num;
use std::ops::{Add, Mul, Div, Sub};
use std::any::TypeId;

#[derive(Debug, PartialEq)]
struct Tensor<T: Num> {
    values: Vec<T>,
    shapes: Vec<usize>,
}

impl<T: Num + Clone> Tensor<T> {
    #[warn(dead_code)]
    fn new(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = Vec::with_capacity(shapes.iter().product());
        Self { values, shapes }
    }
    #[warn(dead_code)]
    fn zero(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vec![T::zero(); shapes.iter().product()];
        Self { values, shapes }
    }
    #[warn(dead_code)]
    fn fill(shapes: Vec<usize>, value: T) -> Self {
        let values: Vec<T> = vec![value; shapes.iter().product()];
        Self { values, shapes }
    }
    #[warn(dead_code)]
    fn is_same_types<L: 'static, U: 'static>(_: &Vec<L>, _: &Vec<U>) -> bool {
        TypeId::of::<L>() == TypeId::of::<U>()
    }
    #[warn(dead_code)]
    fn shapes(&self) -> Vec<usize> {
        self.shapes.clone()
    }
}

// СЛОЖЕНИЕ
impl<T: Num + Clone> Add for Tensor<T> {
    type Output = Tensor<T>;  // Было TEnsor<T> (опечатка)
    
    fn add(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match!");
        let values = self.values.iter()
            .zip(other.values.iter())  // Было valies (опечатка)
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes }
    }
}

impl<T: Num + Clone> Add for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: &Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();  // Было colect (опечатка)
        Tensor { values, shapes: self.shapes.clone() }
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
        Tensor { values, shapes: self.shapes }
    }
}

impl<T: Num + Clone> Add<Tensor<T>> for &Tensor<T> {
    type Output = Tensor<T>;
    
    fn add(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match");  // Было "nust"
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Tensor { values, shapes: self.shapes.clone() }
    }
}

// ВЫЧИТАНИЕ
impl<T: Num + Clone> Sub for Tensor<T> {
    type Output = Tensor<T>;  // Было TEnsor<T>
    
    fn sub(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match!");
        let values = self.values.iter()
            .zip(other.values.iter())  // Было valies
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Tensor { values, shapes: self.shapes }
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
        Tensor { values, shapes: self.shapes.clone() }
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
        Tensor { values, shapes: self.shapes }
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
        Tensor { values, shapes: self.shapes.clone() }
    }
}

fn main() {
    let shap1 = vec![2, 2, 2];  // можно без as usize
    let shap2 = vec![2, 2, 2];

    let t1 = Tensor::fill(shap1, 1);
    let t2 = Tensor::fill(shap2, 2);
    
    // Все варианты работают:
    let _r1 = t1 + t2;  // владение + владение
    
    let t3 = Tensor::fill(vec![2, 2, 2], 3);
    let t4 = Tensor::fill(vec![2, 2, 2], 4);
    let _r2 = &t3 + &t4;  // ссылка + ссылка
    
    let t5 = Tensor::fill(vec![2, 2, 2], 5);
    let t6 = Tensor::fill(vec![2, 2, 2], 6);
    let _r3 = t5 + &t6;  // владение + ссылка
    
    let t7 = Tensor::fill(vec![2, 2, 2], 7);
    let t8 = Tensor::fill(vec![2, 2, 2], 8);
    let _r4 = &t7 + t8;  // ссылка + владение
}