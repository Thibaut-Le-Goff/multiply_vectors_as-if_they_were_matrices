fn mul(mat1: &[i32], mat2: &[i32], column_result: &usize) -> Vec<i32> {

    // the result is stored here
    let mut mat_result: Vec<i32> = Vec::new();

    let number_row_mat2: usize = mat2.len() / column_result;
    let number_row_mat1: usize = mat1.len() / number_row_mat2;
        
    for row in 0..number_row_mat1 {
    // for each row of mat1

        // The number of columns of the second matrix (mat2) is  
        // equal to the number of columns of the matrix as the result :
        for col in 0..*column_result {
        // Multiply the the row of mat1 by the collumn of mat2
            
            let mut result: i32 = 0;
    
            for value in 0..number_row_mat2 {
                let value_mat2: usize = (*column_result * value) + col;
                let value_mat1: usize = (number_row_mat2 * row) + value;

                let pre_result: i32 = mat1[value_mat1] * mat2[value_mat2];
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