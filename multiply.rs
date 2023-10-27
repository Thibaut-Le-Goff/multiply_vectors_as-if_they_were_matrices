fn mul(mat1: &[i32], mat2: &[i32], column_result: usize) {

    let mut result_mat: Vec<i32> = Vec::new();

    let divisor_mat2_for_row: usize = mat2.len() / column_result;

    println!("divisor_mat2_for_row : {:?}", divisor_mat2_for_row);

    //for col in 0..column_result {
    for row in 0..column_result {
        // for each row of the result_mat

        // extract the row from mat1
        let mut vec_row: Vec<i32> = Vec::new();

        for row_mat1 in (row * divisor_mat2_for_row)..((row * divisor_mat2_for_row) + divisor_mat2_for_row) {
            vec_row.push(mat1[row_mat1]);
        }
        
        println!("\nvec_row : {:?}", vec_row);


        //for row in 0..column_result {
        for col in 0..column_result {
            // for each collumn of the result_mat

            // extract the column from mat2
            let mut vec_col: Vec<i32> = Vec::new();
    
            for col_mat2 in (col..mat2.len()).step_by(column_result) {
                vec_col.push(mat2[col_mat2]);
            }

            println!("vec_col : {:?}", vec_col);

            // multiply vec_row by vec_col
            let mut result: i32 = 0;

            for value in 0..vec_col.len() {
                let pre_result: i32 = vec_row[value] * vec_col[value];
                result += pre_result;
            }

            result_mat.push(result);
        }
    }

    println!("\nresult_mat : {:?}", result_mat);
}


fn main() {
    let mat1: Vec<i32> = vec![0, 1, 2, 3];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 3;

    mul(&mat1, &mat2, column_result);
}