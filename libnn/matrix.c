#include "matrix.h"

mat new_empty(size_t row, size_t col)
{
    double *val = (double *)malloc(row * col * sizeof(double));
    mat p = malloc(2 * sizeof(size_t) + sizeof(double *));
    if (val == NULL || p == NULL)
        materr("There is not enough memory space to allocate. ");
    p->row = row;
    p->col = col;
    p->val = val;
    return p;
}

mat new_zeros(size_t row, size_t col)
{
    double *val = (double *)calloc(row * col, sizeof(double));
    mat p = malloc(2 * sizeof(size_t) + sizeof(double *));
    if (val == NULL || p == NULL)
        materr("There is not enough memory space to allocate. ");
    p->row = row;
    p->col = col;
    p->val = val;
    return p;
}

mat new_ones(size_t row, size_t col)
{
    size_t i;
    mat p = new_empty(row, col);
    for (i = 0; i < row * col; i++)
        p->val[i] = 1;
    return p;
}

mat new_eye(size_t size)
{
    size_t i, j;
    mat p = new_empty(size, size);
    for (i = 0; i < size; i++)
        for (j = 0; j < size; j++)
            p->val[i * size + j] = (i == j);
    return p;
}

mat from_array(double val[], size_t row, size_t col)
{
    size_t i;
    mat p = new_empty(row, col);
    for (i = 0; i < row * col; i++)
        p->val[i] = val[i];
    return p;
}

mat from_other(mat pm)
{
    size_t i;
    mat p = new_empty(p->row, p->col);
    for (i = 0; i < p->row * p->col; i++)
        p->val[i] = pm->val[i];
}

void materr(const char *err)
{
    printf("Error: %s\n", err);
    exit(1);
}

void matlog(mat pm)
{
    size_t i, j;
    for (i = 0; i < pm->row; i++, printf("\n"))
        for (j = 0; j < pm->col; j++)
            printf("%.3f\t", pm->val[i * pm->col + j]);
}

void matdel(mat pm)
{
    free(pm->val);
    free(pm);
}

mat matcpy(mat dest, mat src)
{
    size_t i;
    if (dest->row != src->row || dest->col != src->col)
    {
        matdel(dest);
        dest = new_empty(src->row, src->col);
    }
    for (i = 0; i < src->row * src->col; i++)
        dest->val[i] = src->val[i];
    return dest;
}

mat matadd(mat pm, mat pm1, mat pm2)
{
    size_t i;
    if (pm1->row != pm2->row || pm1->col != pm2->col)
        materr("The sizes of the two matrices do not match. ");
    if (pm->row != pm1->row || pm->col != pm1->col)
    {
        matdel(pm);
        pm = new_empty(pm1->row, pm1->col);
    }
    for (i = 0; i < pm->row * pm->col; i++)
        pm->val[i] = pm1->val[i] + pm2->val[i];
    return pm;
}

mat matsub(mat pm, mat pm1, mat pm2)
{
    size_t i;
    if (pm1->row != pm2->row || pm1->col != pm2->col)
        materr("The sizes of the two matrices do not match. ");
    if (pm->row != pm1->row || pm->col != pm1->col)
    {
        matdel(pm);
        pm = new_empty(pm1->row, pm1->col);
    }
    for (i = 0; i < pm->row * pm->col; i++)
        pm->val[i] = pm1->val[i] - pm2->val[i];
    return pm;
}

mat matmul(mat pm, mat pm1, mat pm2)
{
    size_t i;
    if (pm1->row != pm2->row || pm1->col != pm2->col)
        materr("The sizes of the two matrices do not match. ");
    if (pm->row != pm1->row || pm->col != pm1->col)
    {
        matdel(pm);
        pm = new_empty(pm1->row, pm1->col);
    }
    for (i = 0; i < pm->row * pm->col; i++)
        pm->val[i] = pm1->val[i] * pm2->val[i];
    return pm;
}

mat matdiv(mat pm, mat pm1, mat pm2)
{
    size_t i;
    if (pm1->row != pm2->row || pm1->col != pm2->col)
        materr("The sizes of the two matrices do not match. ");
    if (pm->row != pm1->row || pm->col != pm1->col)
    {
        matdel(pm);
        pm = new_empty(pm1->row, pm1->col);
    }
    for (i = 0; i < pm->row * pm->col; i++)
        pm->val[i] = pm1->val[i] / pm2->val[i];
    return pm;
}

mat matdot(mat pm, mat pm1, mat pm2)
{
    size_t i, j, k;
    mat p = pm;
    if (pm1->col != pm2->row)
        materr("The sizes of the two matrices do not match. ");
    if(p == pm1 || pm == pm2)
    {
        p = new_empty(pm1->row, pm2->col);
    }
    if(pm->row !=pm1->row || pm->col!=pm2->col)
    {
        matdel(pm);
        pm = new_empty(pm1->row, pm2->col);
    }
}
