fn mul(mat1: &[i32], mat2: &[i32], column_result: &usize) -> Vec<i32> {

    // the result is stored here
    let mut mat_result: Vec<i32> = Vec::new();

    // We need to segment mat1 and mat2 in order to use 
    // them as matrices.

    // The number of columns of the matrix as the result 
    // is given in the function.

    // We have 2 clues about the relathionship
    // between the matrix as the result and mat1 and mat2 :
    // 1: the number of rows of mat1 is equal to the
    //    number of rows of the matrix as the result

    // 2: the number of columns of mat2 is equal to the
    //    number of column of the matrix as the result

    // We have 1 clue about the relathionship
    // between mat1 and mat2 :
    // 1: the number of rows of mat1 is equal to 
    //    the number of columns of mat2

    // We have 1 clue about the  :
    // the column of a matrix is equal to the lenght 
    // of the vector divided by the rows, the 
    // inverse is true too.

    // we can have the number of row of mat2 by dividing it lentgh
    // by the number of columns of the matrix 
    let divisor_mat2_for_row: usize = mat2.len() / column_result;
    let divisor_mat1_for_row: usize = mat1.len() / divisor_mat2_for_row;
        
    for row in 0..divisor_mat1_for_row {
    // for each row of mat1
    
        // extract the row from mat1
        let mut vec_row: Vec<i32> = Vec::new();

        let first_number_row: usize = row * divisor_mat2_for_row;
    
        for row_mat1 in first_number_row..(first_number_row + divisor_mat2_for_row) {
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
    
            mat_result.push(result);
        }
    }
    
    mat_result
}


fn main() {
    println!("\nTest 1 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    // the matrix as the result must have 3 columns
    let column_result: usize = 3;
    let result1: Vec<i32> = mul(&mat1, &mat2, &column_result);
    println!("\nresult1 : {:?}", result1);


    println!("\nTest 2 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 2;
    let result2: Vec<i32> = mul(&mat1, &mat2, &column_result);
    println!("\nresult2 : {:?}", result2);


    println!("\nTest 3 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3];
    let mat2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 3;
    let result3: Vec<i32> = mul(&mat1, &mat2, &column_result);
    println!("\nresult3 : {:?}", result3);


    println!("\nTest 4 :");
    let mat1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let mat2: Vec<i32> = vec![0, 1, 2, 3];
    let column_result: usize = 2;
    let result4: Vec<i32> = mul(&mat1, &mat2, &column_result);
    println!("\nresult4 : {:?}", result4);
}