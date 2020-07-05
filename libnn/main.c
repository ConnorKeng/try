#include "matrix.h"
#include <string.h>
int main()
{
    double val[] = {0,1,2,3,4,5,6,7,8};
    mat m1 = new_ones(4, 3);
    mat m2 = new_eye(3);
    mat m3 = from_array(val, 3, 3);
    matdiv(m1, m1, m2);
    matlog(m1);
    return 0;
}
