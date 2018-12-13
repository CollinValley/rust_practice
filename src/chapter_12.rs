
#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T
}

use std::ops::Add;

/// Implement add for Complex with generic types
impl<T> Add for Complex<T>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

// Make this even more generic, doesn't require both operands around the +
// be the same type, nor does it required they return the same type
// This isn't the most useful because very few types support this.
/*
impl<L,R,O> Add<Complex<R>> for Complex<L>
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
*/

#[test]
fn test_complex_add() {
    let a = Complex { re: 1, im: 1 };
    let b = Complex { re: 2, im: 2 };
    let c = b + a;
    let d = Complex { re: 3, im: 3 };

    assert_eq!(c.re, d.re);
    assert_eq!(c.im, d.im);
}

use std::ops::Neg;

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex { re: -self.re, im: -self.im }
    }
}

use std::ops::Not;

impl<T, O> Not for Complex<T>
where T: Not<Output=O>
{
    type Output = Complex<O>;
    fn not(self) -> Complex<O> {
        Complex { re: !self.re, im: !self.im }
    }
}

#[test]
fn test_complex_negation() {
    let a = Complex { re: 1, im: 1 };
    let b = !a;
    let c = -a;
    assert_eq!(!a.re, b.re);
    assert_eq!(!a.im, b.im);
    assert_eq!(-a.re, c.re);
    assert_eq!(-a.im, c.im);
}

use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

#[test]
fn test_add_assign() {
    let mut a = Complex { re: 1, im: 1 };
    let b = Complex { re: 2, im: 2 };
    a += b;
    assert_eq!(a.re, 3);
    assert_eq!(a.im, 3);
}

use std::cmp::PartialEq;

impl<T> PartialEq for Complex<T>
    where T: PartialEq
{
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

use std::ops::{ Mul, Sub};

impl<T> Mul for Complex<T>
    where T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Copy
{
    type Output = Self;
    fn mul(self, rhs: Complex<T>) -> Self {
        let real = (self.re * rhs.re) - (self.im * rhs.im);
        let imaginary = (self.re * rhs.im) + (self.im * rhs.re);
        Complex{ re: real, im: imaginary }
    }
}

#[test]
fn test_eq() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });
}

use std::fmt;
use std::cmp::PartialOrd;
use std::default::Default;
use num::Signed;

impl<T> fmt::Display for Complex<T>
    where T: PartialOrd + fmt::Display + Default + Signed
{
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let i_sign = if self.im < T::default() { '-' } else { '+' };
        write!(dest, "{} {} {}i", self.re, i_sign, self.im.abs())
    }
}

#[test]
fn test_fmt_display() {
    let one_twenty = Complex { re: -0.5, im: 0.866 };
    assert_eq!(format!("{}", one_twenty),
               "-0.5 + 0.866i");

    let two_forty = Complex { re: -0.5, im: -0.866 };
    assert_eq!(format!("{}", two_forty),
               "-0.5 - 0.866i");
}
