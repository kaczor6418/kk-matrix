use std::ops::{IndexMut, Index, Add, Sub};
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
