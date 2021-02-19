use crate::matrix::matrix::Matrix;

pub trait AlgebraicOperations {
    fn add_matrix(&self, matrix: &Matrix) -> Matrix;
    fn subtract_matrix(&self, matrix: &Matrix) -> Matrix;
    fn multiply_by_digit(&self, digit: f64) -> Matrix;
    fn multiply_by_vector(&self, values: &Vec<f64>) -> Matrix;
    fn multiply_by_matrix(&self, matrix: &Matrix) -> Matrix;
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

    fn multiply_by_digit(&self, digit: f64) -> Matrix {
        return Matrix::new(
            self.columns_count,
            self.values.iter().map(|value| value * digit).collect(),
        );
    }

    fn multiply_by_vector(&self, values: &Vec<f64>) -> Matrix {
        let mut new_values: Vec<f64> = vec![];
        for index in 0..values.len() {
            new_values.push(self.values[index] * values[index])
        }
        return Matrix::new(self.columns_count, new_values);
    }

    fn multiply_by_matrix(&self, matrix: &Matrix) -> Matrix {
        let mut result: Vec<f64> = vec![0.0; self.rows_count * matrix.columns_count];
        for row_index in 0..self.rows_count {
            for column_index in 0..matrix.columns_count {
                result[row_index * matrix.columns_count + column_index] = self
                    .multiply_row_by_column(
                        self.get_matrix_row(row_index),
                        matrix.get_matrix_column(column_index),
                    );
            }
        }
        return Matrix::new(matrix.columns_count, result);
    }
}
