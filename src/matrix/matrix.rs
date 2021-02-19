pub struct Matrix {
    values: Vec<f64>,
    columns_count: usize,
    rows_count: usize,
}

impl Matrix {
    pub fn new(columns_count: usize, values: Vec<f64>) -> Matrix {
        return Matrix {
            rows_count: values.len() / columns_count,
            values,
            columns_count,
        };
    }

    pub fn new_zeros_matrix(rows_count: usize, columns_count: usize) -> Matrix {
        return Matrix {
            rows_count,
            columns_count,
            values: vec![0.0; rows_count * columns_count],
        };
    }

    pub fn new_identity_matrix(rows_count: usize, columns_count: usize) -> Matrix {
        let mut matrix = Matrix::new_zeros_matrix(rows_count, columns_count);
        let mut column_index = 0;
        for row_index in 0..rows_count {
            matrix[row_index][column_index] = 1.0;
            column_index += 1;
        }
        return matrix;
    }

    pub fn columns_count(&self) -> usize {
        return self.columns_count;
    }

    pub fn rows_count(&self) -> usize {
        return self.rows_count;
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.rows_count, self.columns_count);
    }

    pub fn get_value(&self, row_index: usize, column_index: usize) -> f64 {
        return self.values[row_index * self.columns_count + column_index];
    }

    pub fn set_value(&mut self, row_index: usize, column_index: usize, value: f64) {
        self.values[row_index * self.columns_count + column_index] = value;
    }

    pub fn get_values(&self) -> &Vec<f64> {
        return &self.values;
    }

    pub fn set_values(&mut self, values: Vec<f64>) {
        self.values = values;
    }

    pub fn transpose(&self) -> Matrix {
        let mut values: Vec<f64> = vec![];
        for column_index in 0..self.columns_count {
            values = values
                .into_iter()
                .chain(self.column(column_index).into_iter())
                .collect();
        }
        return Matrix::new(self.rows_count, values);
    }

    fn column(&self, column_index: usize) -> Vec<f64> {
        return self
            .values
            .iter()
            .skip(column_index)
            .step_by(self.columns_count)
            .copied()
            .collect();
    }

    fn row(&self, row_index: usize) -> &[f64] {
        return &self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count];
    }

    fn mutable_row(&mut self, row_index: usize) -> &mut [f64] {
        return &mut self.values
            [row_index * self.columns_count..row_index * self.columns_count + self.columns_count];
    }

    fn multiply_row_by_column(&self, row: &[f64], column: Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for index in 0..row.len() {
            sum += row[index] * column[index];
        }
        return sum;
    }
}

mod matrix_clone;
mod matrix_operators;
mod algebraic_operations;

#[cfg(test)]
mod matrix_test;
