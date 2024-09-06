pub fn matrix_transpose<T: Clone>(src: &[T], width: usize, height: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(height * width);
    for j in 0..width {
        for i in 0..height {
            out.push(src[i * width + j].clone());
        }
    }
    out
}

pub fn print_matrix(matrix: &[char], width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            print!("{}", matrix[width * i + j]);
        }
        println!();
    }
}
