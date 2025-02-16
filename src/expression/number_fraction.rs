use super::error::{EvalError, EvalResult};
use core::fmt;
use std::{
    cmp,
    num::{NonZero, NonZeroU32},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NumberFraction {
    pub top: u32,
    pub bottom: NonZero<u32>,
    pub is_negative: bool,
}

impl NumberFraction {
    pub fn as_whole_number(self) -> Option<i32> {
        match self.bottom.get() {
            1 => Some(self.top_signed()),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.top == 0
    }

    pub fn abs(self) -> Self {
        Self {
            top: self.top,
            bottom: self.bottom,
            is_negative: false,
        }
    }

    pub unsafe fn new_lazy(top: u32, bottom: NonZero<u32>, is_negative: bool) -> Self {
        Self {
            top,
            bottom,
            is_negative,
        }
    }

    pub const fn top_signed(self) -> i32 {
        if self.is_negative {
            (self.top as i32) * -1
        } else {
            self.top as i32
        }
    }

    pub fn whole_number(number: i32) -> Self {
        unsafe {
            NumberFraction::new_lazy(
                number.unsigned_abs(),
                NonZeroU32::new(1).unwrap(),
                number.is_negative(),
            )
        }
    }

    pub fn bottom_u32(self) -> u32 {
        self.bottom.into()
    }

    pub fn with_flipped_sign(self) -> Self {
        let mut result = self;
        result.is_negative ^= true;
        result
    }

    pub fn bottom_i32(self) -> i32 {
        self.bottom_u32() as i32
    }

    pub fn flipped(self) -> EvalResult<Self> {
        Self::new_in_base_form(self.bottom_i32(), self.top_signed())
    }

    pub fn new_in_base_form(top: i32, bottom: i32) -> EvalResult<Self> {
        if bottom == 0 {
            return Err(EvalError::DivisionByZero);
        }

        let mut abs_top = top.unsigned_abs();
        let mut abs_bottom = bottom.unsigned_abs();

        let last_idx = cmp::min(abs_top, abs_bottom);
        for divisor in (2..=last_idx).rev() {
            if abs_top % divisor == 0 && abs_bottom % divisor == 0 {
                abs_top /= divisor;
                abs_bottom /= divisor;
            }
        }

        if abs_top == 0 {
            abs_bottom = 1;
        }

        Ok(unsafe {
            NumberFraction::new_lazy(
                abs_top,
                NonZero::new(abs_bottom).unwrap(),
                top.is_negative() != bottom.is_negative(),
            )
        })
    }

    pub fn pow(self, exponent: i32) -> EvalResult<Self> {
        if self == Self::whole_number(0) && exponent == 0 {
            return Err(EvalError::ZeroToZero);
        }

        let mut result: NumberFraction = self;
        if exponent.is_negative() {
            result = result.flipped()?;
        }
        let exponent: u32 = exponent.unsigned_abs();

        result.top = result.top.pow(exponent);
        result.bottom = NonZero::new(result.bottom_u32().pow(exponent))
            .expect("Non zero value raised to another value should not equal zero");

        Ok(result)
    }
}

impl fmt::Display for NumberFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_negative {
            write!(f, "-(")?;
        }
        write!(f, "{}/{}", self.top, self.bottom.get())?;
        if self.is_negative {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl Add for NumberFraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let top = (self.top_signed() * rhs.bottom_i32()) + (rhs.top_signed() * self.bottom_i32());
        let bottom = self.bottom_i32() * rhs.bottom_i32();
        NumberFraction::new_in_base_form(top, bottom).expect("Both bottoms cannot be zero")
    }
}

impl AddAssign for NumberFraction {
    fn add_assign(&mut self, rhs: Self) {
        let result = *self + rhs;
        *self = result;
    }
}

impl Mul for NumberFraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let top = self.top_signed() * rhs.top_signed();
        let bottom = self.bottom_i32() * rhs.bottom_i32();
        NumberFraction::new_in_base_form(top, bottom).expect("Both bottoms cannot be zero")
    }
}

impl MulAssign for NumberFraction {
    fn mul_assign(&mut self, rhs: Self) {
        let result = *self * rhs;
        *self = result;
    }
}

impl Sub for NumberFraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.with_flipped_sign()
    }
}

impl Div for NumberFraction {
    type Output = EvalResult<Self>;
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_flipped = rhs.flipped()?;
        Ok(self * rhs_flipped)
    }
}

#[cfg(test)]
mod test {
    use std::num::{NonZero, NonZeroU32};

    use crate::expression::error::EvalError;

    use super::NumberFraction;

    fn nz(num: u32) -> NonZeroU32 {
        NonZero::new(num).unwrap()
    }

    #[test]
    fn base_form_1() {
        let frac = NumberFraction::new_in_base_form(10, 5).unwrap();
        assert_eq!(
            frac,
            NumberFraction {
                top: 2,
                bottom: nz(1),
                is_negative: false
            }
        );
        assert_eq!(frac.as_whole_number(), Some(2));
        assert_eq!(frac.top_signed(), 2);
    }

    #[test]
    fn base_form_2() {
        let frac = NumberFraction::new_in_base_form(5, -10).unwrap();
        assert_eq!(
            frac,
            NumberFraction {
                top: 1,
                bottom: nz(2),
                is_negative: true
            }
        );
        assert_eq!(frac.as_whole_number(), None);
        assert_eq!(frac.top_signed(), -1);
    }

    #[test]
    fn base_form_3() {
        let frac = NumberFraction::new_in_base_form(-1234, 5742).unwrap();
        assert_eq!(
            frac,
            NumberFraction {
                top: 617,
                bottom: nz(2871),
                is_negative: true
            }
        );
        assert_eq!(frac.as_whole_number(), None);
        assert_eq!(frac.top_signed(), -617);
    }

    #[test]
    fn base_form_4() {
        let frac = NumberFraction::new_in_base_form(-40, -90).unwrap();
        assert_eq!(
            frac,
            NumberFraction {
                top: 4,
                bottom: nz(9),
                is_negative: false
            }
        );
        assert_eq!(frac.as_whole_number(), None);
        assert_eq!(frac.top_signed(), 4);
    }

    #[test]
    fn flipped_sign() {
        let frac = NumberFraction {
            top: 5,
            bottom: nz(28),
            is_negative: true,
        };
        assert_eq!(
            frac.with_flipped_sign(),
            NumberFraction {
                top: 5,
                bottom: nz(28),
                is_negative: false,
            }
        );
    }

    #[test]
    fn mul_1() {
        let res = NumberFraction::whole_number(3) * NumberFraction::whole_number(5);
        assert_eq!(res.as_whole_number(), Some(15));
        assert_eq!(
            res,
            NumberFraction {
                top: 15,
                bottom: nz(1),
                is_negative: false
            }
        );
    }

    #[test]
    fn mul_2() {
        let res = unsafe {
            NumberFraction::new_lazy(3, nz(4), false) * NumberFraction::new_lazy(8, nz(6), true)
        };
        assert_eq!(res.as_whole_number(), Some(-1));
        assert_eq!(
            res,
            NumberFraction {
                top: 1,
                bottom: nz(1),
                is_negative: true
            }
        );
    }

    #[test]
    fn division_by_zero_detection_1() {
        assert_eq!(
            NumberFraction::new_in_base_form(234, -0),
            Err(EvalError::DivisionByZero)
        );
    }

    #[test]
    fn division_by_zero_detection_2() {
        assert_eq!(
            NumberFraction::new_in_base_form(0, 0),
            Err(EvalError::DivisionByZero)
        );
    }

    #[test]
    fn division_by_zero_detection_3() {
        let frac1 = NumberFraction {
            top: 20,
            bottom: nz(17),
            is_negative: false,
        };
        let frac2 = NumberFraction {
            top: 0,
            bottom: nz(1),
            is_negative: true,
        };
        assert_eq!(frac1 / frac2, Err(EvalError::DivisionByZero));
        assert_eq!(frac2.flipped(), Err(EvalError::DivisionByZero));
    }

    #[test]
    fn add_1() {
        let frac1 = NumberFraction {
            top: 1,
            bottom: nz(3),
            is_negative: false,
        };
        let frac2 = NumberFraction {
            top: 1,
            bottom: nz(2),
            is_negative: false,
        };
        let res = frac1 + frac2;
        assert_eq!(
            res,
            NumberFraction {
                top: 5,
                bottom: nz(6),
                is_negative: false
            }
        );
    }

}
