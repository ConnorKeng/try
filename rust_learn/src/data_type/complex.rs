#![allow(dead_code)]

#[derive(Debug)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl std::clone::Clone for Complex {
    fn clone(&self) -> Self {
        Complex {
            re: self.re,
            im: self.im,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let t = self.re * self.re + self.im * self.im;
        Complex {
            re: (self.re * other.re + self.im * other.im) / t,
            im: (-self.re * other.im + self.im * other.re) / t,
        }
    }
}

impl Complex {
    pub fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    pub fn arg(&self) -> f64 {
        (self.im / self.re).atan()
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re, im }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.re == 0.0 && self.im == 0.0 {
            write!(f, "0")
        } else if self.re == 0.0 {
            write!(f, "{}i", self.im)
        } else if self.im == 0.0 {
            write!(f, "{}", self.re)
        } else if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, self.im.abs())
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
