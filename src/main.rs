use rust_project::Tensor;
fn main() {
    let shap1 = vec![2, 2, 2];
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