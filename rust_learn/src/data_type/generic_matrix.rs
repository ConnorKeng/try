#![allow(dead_code)]

#[derive(Debug,Clone)]
pub struct Matrix<T> {
    row: usize,
    col: usize,
    val: Vec<T>,
}

#[macro_export]
macro_rules! mat {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x.to_vec());
            )*
            Matrix::new(v)
        }
    };
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn new(v: Vec<Vec<T>>) -> Matrix<T> {
        let row = v.len();
        let col = v[0].len();
        let mut val: Vec<T> = Vec::with_capacity(row * col);
        for i in 0..row {
            for j in 0..col {
                val.push(v[i][j].clone());
            }
        }
        Matrix { row, col, val }
    }
    pub fn full(row: usize, col: usize, value: T) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            row,
            col,
            val: vec![value; row * col],
        }
    }

    pub fn eye(size: usize, value: T) -> Matrix<T>
    where
        T: Clone + std::ops::Sub<Output = T>,
    {
        let mut val = Vec::with_capacity(size * size);
        for i in 0..size {
            for j in 0..size {
                val.push(value.clone());
                if i != j {
                    val[i * size + j] = val[i * size + j].clone() - value.clone();
                }
            }
        }
        Matrix {
            row: size,
            col: size,
            val,
        }
    }
}

impl<T> std::ops::Add for Matrix<T>
where
    T: Clone + std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("the sizes of the two matrices do not match");
        }
        for i in 0..self.val.len() {
            self.val[i] = self.val[i].clone() + other.val[i].clone();
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

impl<T> std::ops::Sub for Matrix<T>
where
    T: Clone + std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("the sizes of the two matrices do not match");
        }
        for i in 0..self.val.len() {
            self.val[i] = self.val[i].clone() - other.val[i].clone();
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

impl<T> std::ops::Mul for Matrix<T>
where
    T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.col != other.row {
            panic!("the sizes of the two matrices do not match");
        }
        let mut val = Vec::with_capacity(self.row * other.col);
        let mut pos: usize;
        for i in 0..self.row {
            for j in 0..other.col {
                pos = i * other.col + j;
                val.push(self.val[i * self.col].clone() * other.val[j].clone());
                for k in 1..self.col {
                    val[pos] = val[pos].clone()
                        + self.val[i * self.col + k].clone() * other.val[k * other.col + j].clone();
                }
            }
        }
        Matrix {
            row: self.row,
            col: other.col,
            val,
        }
    }
}

impl<T> std::ops::Mul<T> for Matrix<T>
where
    T: Clone + std::ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(mut self, other: T) -> Self {
        let len = self.row * self.col;
        for i in 0..len {
            self.val[i] = self.val[i].clone() * other.clone();
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

// impl<T> std::ops::Mul<Matrix<T>> for T
// where
//     T: std::ops::Mul<Output = T>,
// {
//     type Output = Matrix<T>;

//     fn mul(self, other: Matrix<T>) -> Matrix<T> {
//         let len = other.row * other.col;
//         for i in 0..len {
//             other.val[i] = other.val[i] * self;
//         }
//         Matrix {
//             row: other.row,
//             col: other.col,
//             val: other.val,
//         }
//     }
// }

impl<T> std::fmt::Display for Matrix<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("\n");
        for i in 0..self.row {
            for j in 0..self.col {
                s.push_str(&format!("{}\t", self.val[i * self.col + j]));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
