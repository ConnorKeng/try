#ifndef _MATRIX_H
#define _MATRIX_H 1

#define mat matrix *

#include <stdlib.h>
#include <malloc.h>

typedef struct
{
    size_t row, col;
    double *val;

} matrix;

mat new_empty(size_t row, size_t col);

mat new_zeros(size_t row, size_t col);

mat new_ones(size_t row, size_t col);

mat new_eye(size_t size);

mat from_array(double val[], size_t row, size_t col);

mat from_other(mat pm);

// void matset(mat *pm, size_t row, size_t col, double val);
// double matget(mat *pm, size_t row, size_t col);

void materr(const char *err);

void matlog(mat pm);

void matdel(mat pm);

mat matcpy(mat dest, mat src);

mat matadd(mat pm, mat pm1, mat pm2);

mat matsub(mat pm, mat pm1, mat pm2);

mat matmul(mat pm, mat pm1, mat pm2);

mat matdiv(mat pm, mat pm1, mat pm2);

mat matdot(mat pm, mat pm1, mat pm2);

#endif // !_MATRIX_H
