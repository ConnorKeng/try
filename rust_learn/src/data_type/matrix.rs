#![allow(dead_code)]

#[derive(Debug,Clone)]
pub struct Matrix {
    row: usize,
    col: usize,
    val: Vec<f64>,
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

impl Matrix {
    pub fn new(v: Vec<Vec<f64>>) -> Matrix {
        let row = v.len();
        let col = v[0].len();
        let mut val = Vec::with_capacity(row * col);
        for i in 0..row {
            for j in 0..col {
                val.push(v[i][j]);
            }
        }
        Matrix { row, col, val }
    }
    pub fn full(row: usize, col: usize, value: f64) -> Matrix {
        Matrix {
            row,
            col,
            val: vec![value; row * col],
        }
    }
    pub fn unit(size: usize) -> Matrix {
        let mut val = Vec::with_capacity(size * size);
        for i in 0..size {
            for j in 0..size {
                val.push(0.0);
                if i == j {
                    val[i * size + j] = 1.0;
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

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row * self.col + col <= self.row * self.col {
            Some(self.val[row * self.col + col])
        } else {
            None
        }
    }
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("the sizes of the two matrices do not match");
        }
        for i in 0..self.val.len() {
            self.val[i] += other.val[i];
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        if self.row != other.row || self.col != other.col {
            panic!("the sizes of the two matrices do not match");
        }
        for i in 0..self.val.len() {
            self.val[i] -= other.val[i];
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.col != other.row {
            panic!("the sizes of the two matrices do not match");
        }
        let mut val = Vec::with_capacity(self.row * other.col);
        let mut pos;
        for i in 0..self.row {
            for j in 0..other.col {
                pos = i * other.col + j;
                val.push(self.val[i * self.col] * other.val[j]);
                for k in 1..self.col {
                    val[pos] += self.val[i * self.col + k] * other.val[k * other.col + j];
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

impl std::ops::Mul<f64> for Matrix {
    type Output = Self;

    fn mul(mut self, other: f64) -> Self {
        let len = self.row * self.col;
        for i in 0..len {
            self.val[i] *= other;
        }
        Matrix {
            row: self.row,
            col: self.col,
            val: self.val,
        }
    }
}

impl std::ops::Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, mut other: Matrix) -> Matrix {
        let len = other.row * other.col;
        for i in 0..len {
            other.val[i] = other.val[i] * self;
        }
        Matrix {
            row: other.row,
            col: other.col,
            val: other.val,
        }
    }
}

impl std::fmt::Display for Matrix {
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
