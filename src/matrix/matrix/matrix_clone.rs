use crate::matrix::matrix::Matrix;

impl Clone for Matrix {
    fn clone(&self) -> Self {
        return Matrix {
            columns_count: self.columns_count,
            rows_count: self.rows_count,
            values: self.values.clone(),
        };
    }
}
