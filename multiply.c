#include <stdio.h>
#include <stdlib.h>

//gcc multiply.c && valgrind./a.out

int* mul(const int *mat1, const int *mat2, const size_t *column_result, const size_t size1, const size_t size2) {
    int *mat_result = (int*)malloc(size1 * sizeof(int));

    size_t number_row_mat2 = size2 / (*column_result);
    size_t number_row_mat1 = size1 / number_row_mat2;

    for (size_t row_mat1 = 0; row_mat1 < number_row_mat1; row_mat1++) {
        for (size_t col_mat2 = 0; col_mat2 < *column_result; col_mat2++) {
            int result = 0;
            for (size_t row_mat2 = 0; row_mat2 < number_row_mat2; row_mat2++) {
                size_t index_mat1 = (number_row_mat2 * row_mat1) + row_mat2;
                size_t index_mat2 = (*column_result * row_mat2) + col_mat2;
                int pre_result = mat1[index_mat1] * mat2[index_mat2];
                result += pre_result;
            }
            mat_result[col_mat2] = result;
        }
    }

    return mat_result;
}

int main() {
    printf("\nlayer 1 :\n");
    int weights_1[] = {0, 1, 2, 3, 4, 5};
    int inputs[] = {0, 1, 2};
    size_t column_result = 1;
    size_t size1 = sizeof(weights_1) / sizeof(weights_1[0]);
    size_t size2 = sizeof(inputs) / sizeof(inputs[0]);
    int *neurones_1 = mul(weights_1, inputs, &column_result, size1, size2);
    printf("\nNumber at the neurones of the layer 1 : ");
    for (size_t i = 0; i < sizeof(neurones_1); i++) {
        printf("%d ", neurones_1[i]);
    }
    printf("\n");

    printf("\nTest 2 :\n");
    int weights_2[] = {0, 1, 2, 3, 4, 5};
    size_t size3 = sizeof(weights_2) / sizeof(weights_2[0]);
    int *neurones_2 = mul(weights_2, neurones_1, &column_result, size3, column_result);
    printf("\nNumber at the neurones of the layer 2 : ");
    for (size_t i = 0; i < sizeof(neurones_2); i++) {
        printf("%d ", neurones_2[i]);
    }
    printf("\n");

    printf("\nTest 3 :\n");
    int mat1[] = {0, 1, 2, 3};
    int mat2[] = {0, 1, 2, 3, 4, 5};
    size_t column_result3 = 3;
    size_t size4 = sizeof(mat1) / sizeof(mat1[0]);
    size_t size5 = sizeof(mat2) / sizeof(mat2[0]);
    int *result3 = mul(mat1, mat2, &column_result3, size4, size5);
    printf("\nresult3 : ");
    for (size_t i = 0; i < sizeof(result3); i++) {
        printf("%d ", result3[i]);
    }
    printf("\n");

    printf("\nTest 4 :\n");
    int mat3[] = {0, 1, 2, 3, 4, 5};
    int mat4[] = {0, 1, 2, 3};
    size_t column_result4 = 2;
    size_t size6 = sizeof(mat3) / sizeof(mat3[0]);
    size_t size7 = sizeof(mat4) / sizeof(mat4[0]);
    int *result4 = mul(mat3, mat4, &column_result4, size6, size7);
    printf("\nresult4 : ");
    for (size_t i = 0; i < sizeof(result4); i++) {
        printf("%d ", result4[i]);
    }
    printf("\n");

    free(neurones_1);
    free(neurones_2);
    free(result3);
    free(result4);

    return 0;
}
