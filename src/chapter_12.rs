
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
