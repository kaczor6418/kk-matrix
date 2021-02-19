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

mod get_value {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_return_value_under_given_position() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = 5.0;
        let matrix = Matrix::new(3, matrix_values);
        let result = matrix.get_value(1, 1);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_return_value_under_given_position_with_overloaded_index_operator() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = 5.0;
        let matrix = Matrix::new(3, matrix_values);
        let result = matrix[1][1];
        assert_eq!(result, expected_result);
    }
}

mod get_values {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_return_all_matrix_values() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(3, matrix_values.clone());
        assert_eq!(matrix.get_values(), &matrix_values);
    }
}

mod set_value {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_set_value_under_given_position() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let row_index = 1;
        let column_index = 1;
        let new_value = 10.0;
        let mut matrix = Matrix::new(3, matrix_values);
        matrix.set_value(row_index, column_index, new_value);
        assert_eq!(matrix.get_value(row_index, column_index), new_value);
    }

    #[test]
    fn should_set_value_under_given_position_with_overloaded_index_operator() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let row_index = 1;
        let column_index = 1;
        let new_value = 10.0;
        let mut matrix = Matrix::new(3, matrix_values);
        matrix[row_index][column_index] = new_value;
        assert_eq!(matrix.get_value(row_index, column_index), new_value);
    }
}

mod set_values {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_set_all_values() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let new_matrix_values = vec![5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let mut matrix = Matrix::new(3, matrix_values);
        matrix.set_values(new_matrix_values.clone());
        assert_eq!(matrix.values, new_matrix_values);
    }
}
