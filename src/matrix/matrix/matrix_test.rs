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

mod columns_count {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_given_number_of_columns() {
        let columns_count = 6;
        let matrix = Matrix::new_zeros_matrix(1, columns_count);
        assert_eq!(matrix.columns_count(), columns_count);
    }
}

mod rows_count {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_calculated_number_of_rows() {
        let columns_count = 2;
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let rows_count = matrix_values.len() / columns_count;
        let matrix = Matrix::new(columns_count, matrix_values);
        assert_eq!(matrix.rows_count(), rows_count);
    }
}

mod size {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_create_matrix_with_calculated_size() {
        let expected_columns_count = 2;
        let expected_rows_count = 3;
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let matrix = Matrix::new(expected_columns_count, matrix_values);
        let (rows_count, columns_count) = matrix.size();
        assert_eq!(expected_rows_count, rows_count);
        assert_eq!(expected_columns_count, columns_count);
    }
}

mod clone {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_be_able_to_clone_matrix() {
        let matrix = Matrix::new_zeros_matrix(2, 2);
        let matrix_clone = matrix.clone();
        assert_eq!(matrix.values, matrix_clone.values);
        assert_eq!(matrix.rows_count, matrix_clone.rows_count);
        assert_eq!(matrix.columns_count, matrix_clone.columns_count);
    }

    #[test]
    fn should_not_change_original_if_copy_was_modified() {
        let matrix = Matrix::new_zeros_matrix(2, 2);
        let matrix_base_clone = matrix.clone();
        let mut matrix_changed_clone = matrix.clone();
        matrix_changed_clone.rows_count = 3;
        matrix_changed_clone.columns_count = 3;
        matrix_changed_clone.values = vec![1.0; 9];
        assert_eq!(matrix.values, matrix_base_clone.values);
        assert_eq!(matrix.rows_count, matrix_base_clone.rows_count);
        assert_eq!(matrix.columns_count, matrix_base_clone.columns_count);
    }
}

mod transpose {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_transpose_2_x_3_into_3_x_2_matrix() {
        let matrix_values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let expected_result = Matrix::new(2, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = matrix.transpose();
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod join_horizontal {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_merge_two_matrix_and_give_matrix_with_same_rows_count_and_sum_of_matrices_columns_count(
    ) {
        let matrix_a = Matrix::new(2, vec![1.0, 1.0, 1.0, 1.0]);
        let matrix_b = Matrix::new(3, vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]);
        let expected_values = vec![1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let result_matrix = matrix_a.join_horizontal(&matrix_b);
        assert_eq!(result_matrix.values, expected_values);
        assert_eq!(result_matrix.rows_count, matrix_a.rows_count);
        assert_eq!(
            result_matrix.columns_count,
            matrix_a.columns_count + matrix_b.columns_count
        );
    }
}

mod join_vertical {
    use crate::matrix::matrix::Matrix;

    #[test]
    fn should_merge_two_matrix_and_give_matrix_with_same_columns_count_and_sum_of_matrices_rows_count(
    ) {
        let matrix_a = Matrix::new(2, vec![1.0, 1.0, 1.0, 1.0]);
        let matrix_b = Matrix::new(2, vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0]);
        let expected_values = vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0];
        let result_matrix = matrix_a.join_vertical(&matrix_b);
        assert_eq!(result_matrix.values, expected_values);
        assert_eq!(
            result_matrix.rows_count,
            matrix_a.rows_count + matrix_b.rows_count
        );
        assert_eq!(result_matrix.columns_count, matrix_a.columns_count);
    }
}

mod add {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_add_each_element_of_matrix_a_and_matrix_b() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, vec![4.0, 4.0, 4.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(3, matrix_b_values);
        let result_matrix = matrix_a.add_matrix(&matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_add_each_element_of_matrix_a_and_matrix_b_with_overloaded_add_operator() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, vec![4.0, 4.0, 4.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(3, matrix_b_values);
        let result_matrix = matrix_a + &matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod subtract {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_subtract_each_element_of_matrix_a_and_matrix_b() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, vec![-2.0, 0.0, 2.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(3, matrix_b_values);
        let result_matrix = matrix_a.subtract_matrix(&matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_subtract_each_element_of_matrix_a_and_matrix_b_with_overloaded_subtract_operator() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![3.0, 2.0, 1.0];
        let expected_result = Matrix::new(3, vec![-2.0, 0.0, 2.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(3, matrix_b_values);
        let result_matrix = matrix_a - &matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod multiply_by_digit {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result =
            Matrix::new(3, matrix_values.iter().map(|value| value * digit).collect());
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = matrix.multiply_by_digit(digit);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit_with_overloaded_mul_right_operator() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result =
            Matrix::new(3, matrix_values.iter().map(|value| value * digit).collect());
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = &matrix * digit;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_all_matrix_values_by_given_digit_with_overloaded_mul_left_operator() {
        let digit = 2.0;
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result =
            Matrix::new(3, matrix_values.iter().map(|value| value * digit).collect());
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = digit * &matrix;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod multiply_by_vector {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_multiply_matrix_values_by_vector_values() {
        let vector = vec![1.0, 2.0, 3.0];
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(3, vec![1.0, 4.0, 9.0]);
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = matrix.multiply_by_vector(&vector);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_values_by_vector_values_with_overloaded_mul_right_operator() {
        let vector = vec![1.0, 2.0, 3.0];
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(3, vec![1.0, 4.0, 9.0]);
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = matrix * &vector;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_values_by_vector_values_with_overloaded_mul_left_operator() {
        let vector = vec![1.0, 2.0, 3.0];
        let matrix_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(3, vec![1.0, 4.0, 9.0]);
        let matrix = Matrix::new(3, matrix_values);
        let result_matrix = vector * &matrix;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod multiply_by_matrix {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_multiply_matrix_with_many_rows_and_columns_if_matrix_rows_count_equal_target_matrix_columns_count(
    ) {
        let matrix_a_values = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let matrix_b_values = vec![2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let expected_result = Matrix::new(2, vec![7.0, 8.0, 14.0, 16.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(2, matrix_b_values);
        let result_matrix = matrix_a.multiply_by_matrix(&matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_with_many_rows_and_columns_if_matrix_rows_count_equal_target_matrix_columns_count_with_overloaded_nul_operator(
    ) {
        let matrix_a_values = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0];
        let matrix_b_values = vec![2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
        let expected_result = Matrix::new(2, vec![7.0, 8.0, 14.0, 16.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(2, matrix_b_values);
        let result_matrix = matrix_a * &matrix_b;
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }

    #[test]
    fn should_multiply_matrix_with_one_row_if_target_matrix_has_one_column() {
        let matrix_a_values = vec![1.0, 2.0, 3.0];
        let matrix_b_values = vec![1.0, 2.0, 3.0];
        let expected_result = Matrix::new(1, vec![14.0]);
        let matrix_a = Matrix::new(3, matrix_a_values);
        let matrix_b = Matrix::new(1, matrix_b_values);
        let result_matrix = matrix_a.multiply_by_matrix(&matrix_b);
        assert_eq!(result_matrix.columns_count, expected_result.columns_count);
        assert_eq!(result_matrix.rows_count, expected_result.rows_count);
        assert_eq!(result_matrix.values, expected_result.values);
    }
}

mod kronecker_product {
    use crate::matrix::matrix::Matrix;
    use crate::matrix::matrix::algebraic_operations::AlgebraicOperations;

    #[test]
    fn should_create_a_product_of_size_matrix_a_rows_count_x_matrix_b_rows_count_xx_matrix_a_columns_count_x_matrix_b_columns_count(
    ) {
        let matrix_a = Matrix::new(2, vec![2.0, 2.0, 2.0, 2.0]);
        let matrix_b = Matrix::new(2, vec![2.0, 2.0, 2.0, 2.0]);
        let expected_values = vec![
            4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0,
        ];
        let result_matrix = matrix_a.kronecker_product(&matrix_b);
        assert_eq!(result_matrix.values, expected_values);
        assert_eq!(
            result_matrix.rows_count,
            expected_values.len() / (matrix_a.rows_count * matrix_b.rows_count)
        );
        assert_eq!(
            result_matrix.columns_count,
            expected_values.len() / (matrix_a.columns_count * matrix_b.columns_count)
        );
    }
}
