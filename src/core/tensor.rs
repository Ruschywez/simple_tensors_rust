use num_traits::Num;
use std::any::TypeId;
// вот оно само величие его тензор

#[derive(Debug, PartialEq)]
pub struct Tensor<T: Num + Clone + Copy> {
    pub(crate) values: Vec<T>,
    pub(crate) shapes: Vec<usize>,
    pub(crate) strides: Vec<usize>,
}

/*
    Базовый функционал
*/
impl<T: Num + Clone + Copy> Tensor<T> {
    pub fn shapes(&self) -> &[usize] {
        &self.shapes
    }
    pub fn strides(&self) -> &[usize] {
        &self.strides
    }
    pub fn values(&self) -> &[T] {
        &self.values
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
/*
    Работа с типом тензора
*/
impl<T: Num + Clone + Copy> Tensor<T> {
    /*
        Методы на проверку ранга тензора т сравнение тензоров
     */
    pub fn rank(&self) -> usize {
        /* ранг тензора это количество измерений */
        self.shapes.len()
    }
    pub fn is_scalar(&self) -> bool {
        // это скаляр?
        self.rank() == 0
    }
    pub fn is_vector(&self) -> bool {
        // это вектор?
        self.rank() == 1
    }
    pub fn is_matrix(&self) -> bool {
        self.rank() == 2
        // это матрица?
    }
    pub fn is_shapes_equal(&self, t2: &Tensor<T>) -> bool {
        self.shapes == t2.shapes
    }
    pub fn is_same_rang(&self, t2: &Tensor<T>) -> bool {
        self.rank() == t2.rank()
    }
    /*
        Работа с содержащимися типами данных T
     */
    
    pub fn dtype(&self) -> &'static str {
        std::any::type_name::<T>()
    }

}
#[warn(unused)]
pub fn is_same_types<L: 'static, U: 'static>(_: &Vec<L>, _: &Vec<U>) -> bool {
    // ф-ия 
    TypeId::of::<L>() == TypeId::of::<U>()
}