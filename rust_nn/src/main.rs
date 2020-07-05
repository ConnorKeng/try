use ndarray::array;
use ndarray::ArrayBase;
use ndarray::Ix;

fn main() {
    let m1 = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let m2 = array![[1, 1, 0], [0, 1, 0], [0, 0, 1]];
    let m3 = m1.dot(&m1);
    println!("{}\n\n{}\n\n{}\n", m1, m2, m3);
}
