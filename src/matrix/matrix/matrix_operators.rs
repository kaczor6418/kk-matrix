use std::ops::{IndexMut, Index};
use crate::matrix::matrix::Matrix;

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row_index: usize) -> &[f64] {
        return self.get_matrix_row(row_index);
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row_index: usize) -> &mut [f64] {
        return self.get_mutable_matrix_row(row_index);
    }
}
