
use num_traits::Num;
#[warn(unused_imports)]
use std::ops::Add; // трейт сложение
use std::any::TypeId; // проверка типов

#[derive(Debug, PartialEq)]
struct Tensor <T: Num>{
    values: Vec<T>, // 24 байт
    shapes: Vec<usize>, // 24 байт
    // 48 байт
}

impl<T: Num + Clone> Tensor<T> {
    #[warn(unused)]
    fn new(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = Vec::with_capacity(shapes.iter().product());
        Self { values, shapes }
    }
    #[warn(unused)]
    fn zero(shapes: Vec<usize>) -> Self {
        let values: Vec<T> = vec![T::zero(); shapes.iter().product()];
        Self { values, shapes }
    }
    // fn fill(shapes: Vec<usize>, value: T) -> Self {
    //     let value: Vec<T> = vec!(value; shapes.iter().product());
        
    // }
    #[warn(unused)]
    fn is_same_types<L: 'static, U: 'static>(_: &Vec<L>, _:&Vec<U>) -> bool {
        if TypeId::of::<L>() == TypeId::of::<U>() {
            true
        } else {
            false
        }
    }
    #[warn(unused)]
    fn shapes(&self) -> Vec<usize> {
        // ф-ия вернет копию t.shapes
        // вызывается через:
        // t.shapes()
        // если нужно получить ссылку на объект используйте:
        // t.shapes <- без скобок ()
        self.shapes.clone()
    }
}
impl<T: Num + Clone> Add for Tensor<T> {
    type Output = Tensor<T>; // Возвращаем сразу Тензор

    fn add(self, other: Tensor<T>) -> Self::Output {
        assert_eq!(self.shapes, other.shapes, "Shapes must match!");
        
        let values = self.values.iter()
            .zip(other.values.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();

        Tensor { values, shapes: self.shapes }
    }
}

fn main() {
    let mut vec1: Tensor<i32> = Tensor::zero(vec!(1, 2, 3));
    let vec2: Tensor<i32> = Tensor::zero(vec!(1, 2, 3));
    vec1 = vec1 + vec2;
    println!("{:?}", vec1);
}
