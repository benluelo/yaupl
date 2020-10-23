#[allow(unused_imports)]
use std::ops::{Add, Sub};

const FRACTIONAL_WRAP: u128 = 100_000_000_000_000_000_000_000_000_000_000_000_000;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Number {
    Num { integral: i128, fractional: u128 },
    Infinity,
    NegativeInfinity,
}

impl Number {
    pub fn from_parts(int: i128, frac: u128) -> Self {
        Number::Num {
            integral: int,
            fractional: frac,
        }
    }

    pub fn zero() -> Self {
        Number::Num {
            integral: 0,
            fractional: 0,
        }
    }

    #[allow(dead_code)]
    pub fn one() -> Self {
        Number::Num {
            integral: 1,
            fractional: 0,
        }
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Number::Num {
                    integral: lhs_integral,
                    fractional: lhs_fractional,
                },
                Number::Num {
                    integral: rhs_integral,
                    fractional: rhs_fractional,
                },
            ) => {
                // add the fractional part of the number
                let frac = lhs_fractional + rhs_fractional;
                // add the integral part of the number
                // check that the number doesn't overflow,
                // if it does then return infinity
                let int = if let Some(int) = lhs_integral.checked_add(rhs_integral) {
                    int
                } else {
                    return Number::Infinity;
                };

                // check if the fraction should carry over
                let wrap = frac >= FRACTIONAL_WRAP;
                if wrap {
                    if lhs_integral.checked_add(1).is_some() {
                        Number::from_parts(int + 1, frac - FRACTIONAL_WRAP)
                    } else {
                        Number::Infinity
                    }
                } else {
                    Number::from_parts(int, frac)
                }
            }
            // 1 + oo = oo
            // oo + 1 = oo
            // oo + oo = oo
            (Number::Num { .. }, Number::Infinity)
            | (Number::Infinity, Number::Num { .. })
            | (Number::Infinity, Number::Infinity) => Number::Infinity,
            // 1 + -oo = -oo
            // -oo + 1 = -oo
            // -oo + -oo = -oo
            (Number::Num { .. }, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::Num { .. })
            | (Number::NegativeInfinity, Number::NegativeInfinity) => Number::NegativeInfinity,
            // oo + -oo = 0
            // -oo + oo = 0
            (Number::Infinity, Number::NegativeInfinity)
            | (Number::NegativeInfinity, Number::Infinity) => Number::zero(),
        }
    }
}

#[cfg(test)]
mod test_number {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Number::from_parts(123456, 98278734873) + Number::from_parts(929, 2334),
            Number::from_parts(124385, 98278737207)
        );
    }

    #[test]
    fn test_wrapping_add() {
        assert_eq!(
            Number::from_parts(10, 99_999_999_999_999_999_999_999_999_999_999_999_999)
                + Number::from_parts(10, 2),
            Number::from_parts(21, 1)
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            Number::from_parts(i128::MAX, 0) + Number::from_parts(1, 0),
            Number::Infinity
        );
    }
}

// impl Sub for Number {
//     type Output = Number;

//     fn sub(self, rhs: Self) -> Self::Output {
//         let frac = self.fractional - rhs.fractional;
//         let int = self.integral - rhs.integral;
//         let wrap = frac > 0;
//         if wrap {
//             Number {
//                 integral: int + 1,
//                 fractional: frac - FRACTIONAL_WRAP,
//             }
//         } else {
//             Number {
//                 integral: int,
//                 fractional: frac,
//             }
//         }
//     }
// }
