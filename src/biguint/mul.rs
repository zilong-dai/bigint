use num_traits::zero;

use crate::biguint::BigDigit;
use crate::biguint::BigUint;
use crate::biguint::DoubleBigDigit;
use crate::biguint::DoubleBigUint;
use crate::biguint::BIG_DIGIT_BITS;
// use crate::biguint::DOUBLE_BIG_DIGIT_BITS;

use core::arch::x86_64 as arch;
use core::ops::{Mul, MulAssign};
use std::result;

use super::add::__add2;
use super::sub::sub2;

use core::cmp::Ordering;

// #[inline]
// pub fn mac_with_carry(a: BigDigit, b: BigDigit, c: BigDigit, acc: &mut DoubleBigDigit) -> BigDigit {
//     *acc += DoubleBigDigit::from(a);
//     *acc += DoubleBigDigit::from(b) * DoubleBigDigit::from(c);
//     let lo = *acc as BigDigit;
//     *acc >>= BIG_DIGIT_BITS;
//     lo
// }

pub fn __mul2(a: BigDigit, b: BigDigit) -> (BigDigit, BigDigit) {
    let res = DoubleBigDigit::from(a) * DoubleBigDigit::from(b);
    (res as BigDigit, (res >> BIG_DIGIT_BITS) as BigDigit)
}

#[inline]
fn mul_with_carry(a: BigDigit, b: BigDigit, acc: &mut BigDigit) -> BigDigit {
    let mut carry = *acc as DoubleBigDigit;
    carry += DoubleBigDigit::from(a) * DoubleBigDigit::from(b);
    let lo = carry as BigDigit;
    carry = carry >> BIG_DIGIT_BITS;
    lo
}

impl<const N: usize> Mul<&BigUint<N>> for BigUint<N> {
    type Output = DoubleBigUint<N>;
    #[inline]
    fn mul(self, other: &BigUint<N>) -> DoubleBigUint<N> {
        // let mut result = DoubleBigUint::<N>::zero();
        let mut result: Vec<BigDigit> = vec![0; 2 * N];

        for i in 0..self.0.len() {
            for j in 0..other.0.len() {
                let (mlow, mhigh) = __mul2(self.0[i], other.0[j]);
                {
                    let (low, mut carry) = result[i + j].overflowing_add(mlow);
                    result[i + j] = low;

                    let mut k = 1;
                    while carry {
                        let (new_low, new_carry) =
                            result[i + j + k].overflowing_add(carry as BigDigit);
                        result[i + j + k] = new_low;
                        carry = new_carry;
                        k += 1;
                    }
                }
                {
                    let (low, mut carry) = result[i + j + 1].overflowing_add(mhigh);
                    result[i + j + 1] = low;

                    let mut k = 1;
                    while carry {
                        let (new_low, new_carry) =
                            result[i + j + 1 + k].overflowing_add(carry as BigDigit);
                        result[i + j + 1 + k] = new_low;
                        carry = new_carry;
                        k += 1;
                    }
                }
            }
        }

        for v in result.iter() {
            println!("{:0x}", v);
        }
        DoubleBigUint {
            high: BigUint::new(result[N..2 * N].try_into().unwrap()),
            low: BigUint::new(result[0..N].try_into().unwrap()),
        }
    }
}

// macro_rules! impl_mul_assign {
//     ($(impl<const N: usize> MulAssign<$Other:ty> for BigUint<N>;)*) => {$(
//         impl<const N: usize> MulAssign<$Other> for BigUint<N> {
//             #[inline]
//             fn mul_assign(&mut self, other: $Other) {
//                 match (&*self.data, &*other.data) {
//                     // multiply by zero
//                     (&[], _) => {},
//                     (_, &[]) => self.set_zero(),
//                     // multiply by a scalar
//                     (_, &[digit]) => *self *= digit,
//                     (&[digit], _) => *self = other * digit,
//                     // full multiplication
//                     (x, y) => *self = mul3(x, y),
//                 }
//             }
//         }
//     )*}
// }
// impl_mul_assign! {
//     impl<const N: usize> MulAssign<BigUint<N>> for BigUint<N>;
//     impl<const N: usize> MulAssign<&BigUint<N>> for BigUint<N>;
// }

// impl<const N: usize> MulAssign<&BigUint<N>> for BigUint<N> {
//     fn mul_assign(&mut self, other: &BigUint<N>) {
//         match (&*self.0, &*other.0) {
//             // multiply by zero
//             (&[], _) => {}
//             (_, &[]) => self.set_zero(),
//             // multiply by a scalar
//             (_, &[digit]) => *self *= digit,
//             (&[digit], _) => *self = other * digit,
//             // full multiplication
//             (x, y) => *self = mul3(x, y),
//         }
//     }
// }

// impl<const N: usize> MulAssign<BigUint<N>> for BigUint<N>{
//     fn mul_assign(&mut self, other: $Other) {
//         match (&*self.data, &*other.data) {
//             // multiply by zero
//             (&[], _) => {},
//             (_, &[]) => self.set_zero(),
//             // multiply by a scalar
//             (_, &[digit]) => *self *= digit,
//             (&[digit], _) => *self = other * digit,
//             // full multiplication
//             (x, y) => *self = mul3(x, y),
//         }
//     }

// }
