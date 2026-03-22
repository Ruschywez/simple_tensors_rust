pub fn compute_strides(shape: &[usize]) -> Vec<usize> {
    /*
        Возможно выносить эту функцию в отдельный файл
        было опрометчивым решением. Тем не менее оно работает
     */
    let mut strides = vec![1; shape.len()];
    for i in (0..shape.len() - 1).rev() {
         strides[i] = strides[i+1] * shape[i+1]
    }
    strides
}