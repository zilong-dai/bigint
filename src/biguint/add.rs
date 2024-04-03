// #[cfg(not(u64_digit))]
// use super::u32_from_u128;
use crate::biguint::BigUint;
use crate::biguint::BigDigit;


// use crate::big_digit::{self, BigDigit};
// use crate::UsizePromotion;

use core::ops::{Add, AddAssign};

use core::arch::x86_64 as arch;


#[inline]
fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    // Safety: There are absolutely no safety concerns with calling `_addcarry_u64`.
    // It's just unsafe for API consistency with other intrinsics.
    unsafe { arch::_addcarry_u64(carry, a, b, out) }
}

#[inline]
pub fn __add2(a: &mut [BigDigit], b: &[BigDigit]) -> BigDigit {
    debug_assert!(a.len() >= b.len());

    let mut carry = 0;
    let (a_lo, a_hi) = a.split_at_mut(b.len());

    for (a, b) in a_lo.iter_mut().zip(b) {
        carry = adc(carry, *a, *b, a);
    }

    if carry != 0 {
        for a in a_hi {
            carry = adc(carry, *a, 0, a);
            if carry == 0 {
                break;
            }
        }
    }
    assert_eq!(carry, 0, "addition overflow");
    carry as BigDigit
}

impl<const N: usize> Add<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}
impl<const N: usize> AddAssign<&BigUint<N>> for BigUint<N> {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        let carry = __add2(&mut self.0[..], &other.0[..]);
        assert_eq!(carry, 0, "addition overflow");
    }
}

impl<const N: usize> Add<u32> for BigUint<N> {
    type Output = BigUint<N>;
    #[inline]
    fn add(mut self, other: u32) -> BigUint<N> {
        self += other;
        self
    }
}

impl<const N: usize> AddAssign<u32> for BigUint<N> {
    #[inline]
    fn add_assign(&mut self, other: u32) {
        if other != 0 {
            __add2(&mut self.0, &[other as BigDigit]);
        }
    }
}

impl<const N: usize> Add<u64> for BigUint<N> {
    type Output = BigUint<N>;

    #[inline]
    fn add(mut self, other: u64) -> Self {
        self += other;
        self
    }
}

impl<const N: usize> AddAssign<u64> for BigUint<N> {
    #[inline]
    fn add_assign(&mut self, other: u64) {
        if other != 0 {
            __add2(&mut self.0, &[other as BigDigit]);
        }
    }
}

// impl Add<u128> for BigUint {
//     type Output = BigUint;

//     #[inline]
//     fn add(mut self, other: u128) -> BigUint {
//         self += other;
//         self
//     }
// }

// impl AddAssign<u128> for BigUint {
//     #[cfg(not(u64_digit))]
//     #[inline]
//     fn add_assign(&mut self, other: u128) {
//         if other <= u128::from(u64::max_value()) {
//             *self += other as u64
//         } else {
//             let (a, b, c, d) = u32_from_u128(other);
//             let carry = if a > 0 {
//                 while self.data.len() < 4 {
//                     self.data.push(0);
//                 }
//                 __add2(&mut self.data, &[d, c, b, a])
//             } else {
//                 debug_assert!(b > 0);
//                 while self.data.len() < 3 {
//                     self.data.push(0);
//                 }
//                 __add2(&mut self.data, &[d, c, b])
//             };

//             if carry != 0 {
//                 self.data.push(carry);
//             }
//         }
//     }

//     #[cfg(u64_digit)]
//     #[inline]
//     fn add_assign(&mut self, other: u128) {
//         let (hi, lo) = big_digit::from_doublebigdigit(other);
//         if hi == 0 {
//             *self += lo;
//         } else {
//             while self.data.len() < 2 {
//                 self.data.push(0);
//             }

//             let carry = __add2(&mut self.data, &[lo, hi]);
//             if carry != 0 {
//                 self.data.push(carry);
//             }
//         }
//     }
// }

// impl CheckedAdd for BigUint {
//     #[inline]
//     fn checked_add(&self, v: &BigUint) -> Option<BigUint> {
//         Some(self.add(v))
//     }
// }

// impl_sum_iter_type!(BigUint);
