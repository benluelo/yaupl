use std::ops::{Add, Sub};

const fractional_wrap: u128 = 100_000_000_000_000_000_000_000_000_000_000_000_000;

pub(crate) struct Number {
    integral: i128,
    fractional: u128,
}

impl Number {}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let frac = self.fractional + rhs.fractional;
        let int = self.integral + rhs.integral;
        let wrap = frac > fractional_wrap;
        if wrap {
            Number {
                integral: int + 1,
                fractional: frac - fractional_wrap,
            }
        } else {
            Number {
                integral: int,
                fractional: frac,
            }
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        let frac = self.fractional - rhs.fractional;
        let int = self.integral - rhs.integral;
        let wrap = frac > 0;
        if wrap {
            Number {
                integral: int + 1,
                fractional: frac - fractional_wrap,
            }
        } else {
            Number {
                integral: int,
                fractional: frac,
            }
        }
    }
}
