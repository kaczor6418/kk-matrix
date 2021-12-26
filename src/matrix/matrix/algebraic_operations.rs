use crate::matrix::matrix::Matrix;
use std::sync::{Arc, Mutex};
use crossbeam::thread;

pub trait AlgebraicOperations {
    fn add_matrix(&self, matrix: &Matrix) -> Matrix;
    fn subtract_matrix(&self, matrix: &Matrix) -> Matrix;
    fn multiply_by_digit(&self, digit: f64) -> Matrix;
    fn multiply_by_vector(&self, values: &Vec<f64>) -> Matrix;
    fn multiply_by_matrix(&self, matrix: &Matrix) -> Matrix;
    fn kronecker_product(&self, matrix: &Matrix) -> Matrix;
    fn parallel_kronecker_product(&self, matrix: &Matrix) -> Matrix;
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
                        self.row(row_index),
                        matrix.column(column_index),
                    );
            }
        }
        return Matrix::new(matrix.columns_count, result);
    }

    fn kronecker_product(&self, matrix: &Matrix) -> Matrix {
        let product_rows = self.rows_count * matrix.rows_count;
        let product_columns_count = self.columns_count * matrix.columns_count;
        let mut product = Matrix::new_zeros_matrix(product_rows, product_columns_count);
        for m1_row_index in 0..self.rows_count {
            for m1_column_index in 0..self.columns_count {
                for m2_row_index in 0..matrix.rows_count {
                    for m2_column_index in 0..matrix.columns_count {
                        let product_row_index = m1_row_index * matrix.rows_count + m2_row_index;
                        let product_column_index =
                            m1_column_index * matrix.columns_count + m2_column_index;
                        product[product_row_index][product_column_index] = self[m1_row_index]
                            [m1_column_index]
                            * matrix[m2_row_index][m2_column_index];
                    }
                }
            }
        }
        return product;
    }

    fn parallel_kronecker_product(&self, matrix: &Matrix) -> Matrix {

        // Create a new thread scope and make it own the locals
        thread::scope(move |scope| {
            let product_rows = self.rows_count * matrix.rows_count;
            let product_columns_count = self.columns_count * matrix.columns_count;
            let product = Arc::new(Mutex::new(Matrix::new_zeros_matrix(
                product_rows,
                product_columns_count,
            )));

            let mut handles = vec![];
            for m1_row_index in 0..self.rows_count {
                let product = Arc::clone(&product);

                // spawn a new thread inside the scope which owns the data it needs to borrow
                handles.push(scope.spawn(move |_| {
                    for m1_column_index in 0..self.columns_count {
                        for m2_row_index in 0..matrix.rows_count {
                            for m2_column_index in 0..matrix.columns_count {
                                let product_row_index =
                                    m1_row_index * matrix.rows_count + m2_row_index;
                                let product_column_index =
                                    m1_column_index * matrix.columns_count + m2_column_index;
                                let mut prod = product.lock().unwrap();
                                (*prod)[product_row_index][product_column_index] = self[m1_row_index]
                                    [m1_column_index]
                                    * matrix[m2_row_index][m2_column_index];
                            }
                        }
                    }
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }

            // probably need a new local binding here. For... reasons...
            let product = product.lock().unwrap().clone();
            product
        }).unwrap()
    }
}
