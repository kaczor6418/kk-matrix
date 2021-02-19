use std::ops::{IndexMut, Index, Add, Sub, Mul};
use crate::matrix::matrix::Matrix;
use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

impl Add<&Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, matrix: &Matrix) -> Matrix {
        return self.add_matrix(matrix);
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;
    fn add(self, matrix: &Matrix) -> Matrix {
        return self.add_matrix(matrix);
    }
}

impl Sub<&Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, matrix: &Matrix) -> Matrix {
        return self.subtract_matrix(matrix);
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;
    fn sub(self, matrix: &Matrix) -> Matrix {
        return self.subtract_matrix(matrix);
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, digit: f64) -> Matrix {
        return self.multiply_by_digit(digit);
    }
}

impl Mul<&Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, matrix: &Matrix) -> Matrix {
        return matrix.multiply_by_digit(self);
    }
}

impl Mul<&Vec<f64>> for Matrix {
    type Output = Matrix;
    fn mul(self, values: &Vec<f64>) -> Matrix {
        return self.multiply_by_vector(values);
    }
}

impl Mul<&Matrix> for Vec<f64> {
    type Output = Matrix;
    fn mul(self, matrix: &Matrix) -> Matrix {
        return matrix.multiply_by_vector(&self);
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, matrix: &Matrix) -> Matrix {
        return self.multiply_by_matrix(matrix);
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, matrix: &Matrix) -> Matrix {
        return self.multiply_by_matrix(matrix);
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row_index: usize) -> &[f64] {
        return self.row(row_index);
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row_index: usize) -> &mut [f64] {
        return self.mutable_row(row_index);
    }
}
