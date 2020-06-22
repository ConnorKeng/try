mod data_type;

use data_type::Complex;
use data_type::Matrix;

fn main() {
    let m = mat![[Complex::new(1.0, 2.0), Complex::new(1.0, 2.0)],
                 [Complex::new(1.0, 2.0), Complex::new(1.0, 2.0)],
                 [Complex::new(1.0, 2.0), Complex::new(1.0, 2.0)]];
    println!("{}", m.clone() * Matrix::full(2, 2, Complex::new(2.0, 3.0)));
}
