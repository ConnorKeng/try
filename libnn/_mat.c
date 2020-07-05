#include "_mat.h"

mat *mat_from_array(double *array, size_t row, size_t col)
{
    double *v = (double *)malloc(row * col * sizeof(double));
    memcpy(v, array, row * col * sizeof(double));
    mat *p = malloc(2 * sizeof(size_t) + sizeof(double *));
    p->row = row;
    p->col = col;
    p->v = v;
    return p;
}

mat *mat_empty(size_t row, size_t col)
{
    double *v = (double *)malloc(row * col * sizeof(double));
    mat *p = malloc(2 * sizeof(size_t) + sizeof(double *));
    p->row = row;
    p->col = col;
    p->v = v;
    return p;
}

mat *mat_zeros(size_t row, size_t col)
{
    double *v = (double *)calloc(row * col, sizeof(double));
    mat *p = malloc(2 * sizeof(size_t) + sizeof(double *));
    p->row = row;
    p->col = col;
    p->v = v;
    return p;
}

mat *mat_ones(size_t row, size_t col)
{
    size_t i;
    mat *p = mat_empty(row, col);
    for (i = 0; i < row * col; i++)
    {
        p->v[i] = 1;
    }
    return p;
}

mat *mat_eye(size_t size)
{
    size_t i, j;
    mat *p = mat_zeros(size, size);
    for (i = 0; i < size; i++)
    {
        for (j = 0; j < size; j++)
        {
            p->v[i * size + j] = (i == j);
        }
    }
    return p;
}

double mat_get(mat *p, size_t row, size_t col)
{
    return p->v[row * p->col + col];
}

void mat_del(mat *p)
{
    free(p->v);
    free(p);
}

void mat_dot(mat *p, mat *p1, mat *p2)
{
}

void mat_print(mat *p)
{
    size_t i, j;
    for (i = 0; i < p->row; i++)
    {
        for (j = 0; j < p->col; j++)
        {
            printf("%.3f\t", p->v[i * p->col + j]);
        }
        printf("\n");
    }
}