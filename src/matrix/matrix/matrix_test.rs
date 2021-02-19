mod creating_matrix {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_three_columns_and_two_rows_and_fill_with_given_values() {
        let columns_count = 3;
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(columns_count, values.clone());
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.rows_count, values.len() / columns_count);
        assert_eq!(matrix.values, values);
    }

    #[test]
    fn should_create_matrix_of_zeros_with_given_number_of_rows_and_columns() {
        let columns_count = 3;
        let rows_count = 3;
        let matrix = Matrix::new_zeros_matrix(rows_count, columns_count);
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.rows_count, rows_count);
        assert_eq!(matrix.values, vec![0.0; rows_count * columns_count]);
    }

    #[test]
    fn should_create_identity_matrix_with_given_number_of_rows_and_columns() {
        let columns_count = 3;
        let rows_count = 3;
        let matrix = Matrix::new_identity_matrix(rows_count, columns_count);
        let values_sum: f64 = matrix.values.iter().sum();
        assert_eq!(matrix.columns_count, columns_count);
        assert_eq!(matrix.rows_count, rows_count);
        assert_eq!(values_sum, 3.0);
        assert_eq!(matrix[0][0], 1.0);
        assert_eq!(matrix[1][1], 1.0);
        assert_eq!(matrix[2][2], 1.0);
    }
}
