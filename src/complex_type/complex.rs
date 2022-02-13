use std::ops::Add;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im}
    }
}

impl<T> From<(T,T)> for Complex<T> {
    fn from(value: (T,T)) -> Complex<T> {
        Complex { re: value.0, im: value.1}
    }
}