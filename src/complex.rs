use std::{
    fmt::Display,
    ops::{Add, Mul},
};

#[derive(Debug, Clone, Copy)]
pub struct C {
    pub im: f64,
    pub re: f64,
}

impl Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + i * {}", self.re, self.im)
    }
}

impl C {
    pub fn norm(self) -> f64 {
        self.im * self.im + self.re * self.re
    }
}

impl From<(f64, f64)> for C {
    fn from(value: (f64, f64)) -> Self {
        C {
            re: value.0,
            im: value.1,
        }
    }
}

// (a + ib) * (u + iw)
// (a * u + i * a * w + i * b * u - b * w)
// (a * u - b * w) + i (a * w + b * u)

impl Mul for C {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        C {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Add for C {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        C {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
