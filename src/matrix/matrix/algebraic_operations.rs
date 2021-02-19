use crate::matrix::matrix::Matrix;

pub trait AlgebraicOperations {
    fn add_matrix(&self, matrix: &Matrix) -> Matrix;
    fn subtract_matrix(&self, matrix: &Matrix) -> Matrix;
}

impl AlgebraicOperations for Matrix {
    fn add_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            self.values
                .iter()
                .map(|value| value + matrix_iter.next().unwrap())
                .collect(),
        );
    }

    fn subtract_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut matrix_iter = matrix.values.iter();
        return Matrix::new(
            matrix.columns_count,
            self.values
                .iter()
                .map(|value| value - matrix_iter.next().unwrap())
                .collect(),
        );
    }
}
