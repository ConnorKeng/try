mod data_type;

use data_type::Complex;
use data_type::Matrix;

fn main() {
    let m = mat![ [1.0,2.0],
                  [3.0,4.0] ];
    println!("{}", m.clone()*Matrix::unit(2));

}
