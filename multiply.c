#include <stdio.h>
#include <stdlib.h>

//gcc multiply.c && valgrind./a.out

int* mul(const int *mat1, const int *mat2, const int *column_result, const size_t size1, const size_t size2) {
    int number_row_mat2 = size2 / (*column_result);

    int number_row_mat1 = size1 / number_row_mat2;

    // the number of rows of mat1 is equal to the number of 
    // rows of the matrix as the result
    int *mat_result = (int*)malloc(((*column_result) * number_row_mat1) * sizeof(int));
    //printf("\nle resultat doit contenir %d nombres", sizeof(*mat_result));

    for (int row_mat1 = 0; row_mat1 < number_row_mat1; row_mat1++) {

        // The number of columns of the second matrix (mat2) is  
        // equal to the number of columns of the matrix as the result :
        for (int col_mat2 = 0; col_mat2 < *column_result; col_mat2++) {

            int result = 0;

            for (int row_mat2 = 0; row_mat2 < number_row_mat2; row_mat2++) {

                // The number columns of mat1 is equal
                // to the number of rows of mat2 :
                int index_mat1 = (number_row_mat2 * row_mat1) + row_mat2;

                // The number of columns of the second matrix (mat2) is equal 
                // to the number of columns of the matrix as the result :                
                int index_mat2 = (*column_result * row_mat2) + col_mat2;

                int pre_result = mat1[index_mat1] * mat2[index_mat2];
                result += pre_result;
            }
            
            // The number of columns of the second matrix (mat2) is equal 
            // to the number of columns of the matrix as the result : 

            // the number of rows of mat1 is equal to the number of 
            // rows of the matrix as the result
            mat_result[(*column_result * row_mat1) + col_mat2] = result;
        }
    }

    return mat_result;
}

int main() {

    printf("\nlayer 1 :\n");
    int weights_1[] = {0, 1, 2, 3, 4, 5};
    int inputs[] = {0, 1, 2};

    int column_result = 1; // 1 because we want a vector

    size_t size1 = sizeof(weights_1) / sizeof(weights_1[0]);
    size_t size2 = sizeof(inputs) / sizeof(inputs[0]);
    int *neurones_1 = mul(weights_1, inputs, &column_result, size1, size2);
    //int *neurones_1 = mul(weights_1, inputs, &column_result);

    int iteratore_layer_1 = size1 / size2;
    printf("\nNumber at the neurones of the layer 1 : ");
    for (int i = 0; i < iteratore_layer_1; i++) {
        printf("%d ", neurones_1[i]);
    }
    printf("\n");

    printf("\nlayer 2 :\n");
    int weights_2[] = {0, 1, 2, 3, 4, 5};
    size_t size3 = sizeof(weights_2) / sizeof(weights_2[0]);
    size_t size4 = sizeof(neurones_1) / sizeof(neurones_1[0]);
    int *neurones_2 = mul(weights_2, neurones_1, &column_result, size3, size4);
    //int *neurones_2 = mul(weights_2, neurones_1, &column_result);

    int iteratore_layer_2 = size3 / size4;
    printf("\nNumber at the neurones of the layer 2 : ");
    for (int i = 0; i < iteratore_layer_2; i++) {
        printf("%d ", neurones_2[i]);
    }
    printf("\n");

    free(neurones_1);
    free(neurones_2);

    return 0;
}
