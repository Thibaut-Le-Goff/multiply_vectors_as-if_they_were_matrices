fn mul(mat1: &[i32], mat2: &[i32], column_result: &usize) -> Vec<i32> {
// fn mul(weights_n: &[i32], outputs_n_minus_1: &[i32], default_value_one: &usize) -> Vec<i32> {

    // the result is stored here
    let mut mat_result: Vec<i32> = Vec::new();

    let number_row_mat2: usize = mat2.len() / column_result;
    let number_row_mat1: usize = mat1.len() / number_row_mat2;
        
    for row_mat1 in 0..number_row_mat1 {
    // for each row of mat1

        // The number of columns of the second matrix (mat2) is  
        // equal to the number of columns of the matrix as the result :
        for col_mat2 in 0..*column_result {
        // for each collumn of mat2
            
            let mut result: i32 = 0;
    
            for row_mat2 in 0..number_row_mat2 {
            // for each row of mat2

                // The number columns of mat1 is equal
                // to the number of rows of mat2 :
                let index_mat1: usize = (number_row_mat2 * row_mat1) + row_mat2;

                // The number of columns of the second matrix (mat2) is equal 
                // to the number of columns of the matrix as the result :
                let index_mat2: usize = (*column_result * row_mat2) + col_mat2;

                let pre_result: i32 = mat1[index_mat1] * mat2[index_mat2];
                result += pre_result;
            }
    
            mat_result.push(result);
        }
    }
    
    mat_result
    // neuron outputs n
}


fn main() {
    println!("\nlayer 1 :");
    let weights_1: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let inputs: Vec<i32> = vec![0, 1, 2];
    // the matrix as the result must have 3 columns
    let column_result: usize = 1; // we want to get vector, so with only 1 column
    let neurones_1: Vec<i32> = mul(&weights_1, &inputs, &column_result);
    println!("\nNumber at the neurones of the layer 1 : {:?}", neurones_1);


    println!("\nlayer 2 :");
    let weights_2: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
    let column_result: usize = 1; // we want to get vector, so with only 1 column
    let neurones_2: Vec<i32> = mul(&weights_2, &neurones_1, &column_result);
    println!("\nNumber at the neurones of the layer 2 : {:?}", neurones_2);


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