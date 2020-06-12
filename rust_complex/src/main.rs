fn main() {
    let c1 = Complex { re: 1.0, im: 1.0 };
    let c2 = Complex { re: 1.0, im: 1.0 };
    println!("{}", c1.mul(&c2));
}

// struct Array<T> {

// }

struct Complex {
    re: f64,
    im: f64,
}

#[allow(dead_code)]
impl Complex {
    fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    fn arg(&self) -> f64 {
        (self.im / self.re).atan()
    }
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    fn sub(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
    fn mul(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    fn div(&self, other: &Complex) -> Complex {
        let t = self.re * self.re + self.im * self.im;
        Complex {
            re: (self.re * other.re + self.im * other.im) / t,
            im: (-self.re * other.im + self.im * other.re) / t,
        }
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
            write!(f, "{} - {}i", self.re, self.im.abs())
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}
