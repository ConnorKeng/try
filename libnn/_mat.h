#include <malloc.h>
#include <string.h>
#include <errno.h>
#include <stdio.h>

typedef struct
{
    size_t row, col;
    double *v;

} mat;

mat *mat_from_array(double *array, size_t row, size_t col);

mat *mat_empty(size_t row, size_t col);

mat *mat_zeros(size_t row, size_t col);

mat *mat_ones(size_t row, size_t col);

mat *mat_eye(size_t size);

double mat_get(mat *p, size_t row, size_t col);

void mat_del(mat* p);

void mat_dot(mat *p, mat *p1, mat *p2);

void mat_print(mat *p);
