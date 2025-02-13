use core::fmt;
use std::{
    cmp,
    num::{NonZero, NonZeroU32},
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn flipped(self) -> Option<Self> {
        Self::new_in_base_form(self.bottom_i32(), self.top_signed())
    }

    pub fn new_in_base_form(top: i32, bottom: i32) -> Option<Self> {
        if bottom == 0 {
            return None;
        }

        let mut abs_top = top.unsigned_abs();
        let mut abs_bottom = bottom.unsigned_abs();

        let last_idx = cmp::min(abs_top, abs_bottom) / 2;
        for divisor in (2..=last_idx).rev() {
            if abs_top % divisor == 0 && abs_bottom % divisor == 0 {
                abs_top /= divisor;
                abs_bottom /= divisor;
            }
        }

        if abs_top == 0 {
            abs_bottom = 1;
        }

        Some(unsafe {
            NumberFraction::new_lazy(
                abs_top,
                NonZero::new(abs_bottom).unwrap(),
                top.is_negative() != top.is_negative(),
            )
        })
    }

    pub fn pow(self, exponent: Self) -> Option<Self> {
        if self == Self::whole_number(0) && exponent == Self::whole_number(0) {
            return None;
        }

        let exponent: i32 = if exponent.bottom_i32() == 1 {
            exponent.top_signed()
        } else {
            return None;
        };

        let mut result: NumberFraction = self;
        if exponent.is_negative() {
            result = result.flipped()?;
        }
        let exponent: u32 = exponent.unsigned_abs();

        result.top = result.top.pow(exponent);
        result.bottom = NonZero::new(result.bottom_u32().pow(exponent))?;

        Some(result)
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
        NumberFraction::new_in_base_form(top, bottom).expect("Both bottom cannot be zero")
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
    type Output = Option<Self>; // TODO: this should be Result
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_flipped = rhs.flipped()?;
        Some(self * rhs_flipped)
    }
}
