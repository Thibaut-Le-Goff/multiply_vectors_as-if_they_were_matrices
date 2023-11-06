fn mul(mat1: &[i32], mat2: &[i32], column_result: &usize) -> Vec<i32> {

    // the result is stored here
    let mut result_mat: Vec<i32> = Vec::new();

    // we nned to segment mat1 and mat2 in order use them 
    // as matrices.

    // the number of column of the matrix as the result
    // is given in the function

    // We have 2 clues about the relathionship
    // between the matrix as the result and mat1 and mat2 :
    // 1: the number of column of mat2 is equal to the
    //    number of column of the matrix as the result

    // 2: the number of rows of mat1 is equal to the
    //    number of rows of the matrix as the result

    // We have 1 clue about the relathionship
    // between mat1 and mat2 :
    // 1: the number of rows of mat2 is equal to 
    //    the number of columns of mat1 

    let divisor_mat2_for_row: usize = mat2.len() / column_result;
    let divisor_mat1_for_col: usize = mat1.len() / divisor_mat2_for_row;
    
    //println!("divisor_mat2_for_row : {:?}", divisor_mat2_for_row);
    
    for row in 0..divisor_mat1_for_col {
    // for each row of mat1
    
        // extract the row from mat1
        let mut vec_row: Vec<i32> = Vec::new();
    
        for row_mat1 in (row * divisor_mat2_for_row)..((row * divisor_mat2_for_row) + divisor_mat2_for_row) {
            vec_row.push(mat1[row_mat1]);
        }
            
        println!("\nvec_row : {:?}", vec_row);
    
        for col in 0..*column_result {
        // for each collumn of mat2
    
            // extract the column from mat2
            let mut vec_col: Vec<i32> = Vec::new();
        
            for col_mat2 in (col..mat2.len()).step_by(*column_result) {
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
    result_mat
}


fn main() {
    println!("\nTest 1 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    // the matrix as the result must have 3 columns
    let column_result: usize = 3;
    let result1: Vec<i32> = mul(&mat1, &mat2, &column_result);

    println!("\nTest 2 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 1;
    let result2: Vec<i32> = mul(&mat1, &mat2, &column_result);

    println!("\nTest 3 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 3;
    let result3: Vec<i32> = mul(&mat1, &mat2, &column_result);

    println!("\nTest 4 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3];
    let column_result: usize = 2;
    let result4: Vec<i32> = mul(&mat1, &mat2, &column_result);
}