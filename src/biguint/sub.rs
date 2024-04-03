
use core::arch::x86_64 as arch;
use core::cmp::Ordering::{Equal, Greater, Less};

use core::ops::{Sub, SubAssign};

use crate::biguint::BigUint;
use crate::biguint::BigDigit;


#[inline]
fn sbb(borrow: u8, a: u64, b: u64, out: &mut u64) -> u8 {
    // Safety: There are absolutely no safety concerns with calling `_subborrow_u64`.
    // It's just unsafe for API consistency with other intrinsics.
    unsafe { arch::_subborrow_u64(borrow, a, b, out) }
}

pub(super) fn sub2(a: &mut [BigDigit], b: &[BigDigit]) {
    let mut borrow = 0;

    let len = Ord::min(a.len(), b.len());
    let (a_lo, a_hi) = a.split_at_mut(len);
    let (b_lo, b_hi) = b.split_at(len);

    for (a, b) in a_lo.iter_mut().zip(b_lo) {
        borrow = sbb(borrow, *a, *b, a);
    }

    if borrow != 0 {
        for a in a_hi {
            borrow = sbb(borrow, *a, 0, a);
            if borrow == 0 {
                break;
            }
        }
    }

    // note: we're _required_ to fail on underflow
    assert!(
        borrow == 0 && b_hi.iter().all(|x| *x == 0),
        "Cannot subtract b from a because b is larger than a."
    );
}

// Only for the Sub impl. `a` and `b` must have same length.
#[inline]
fn __sub2rev(a: &[BigDigit], b: &mut [BigDigit]) -> u8 {
    debug_assert!(b.len() == a.len());

    let mut borrow = 0;

    for (ai, bi) in a.iter().zip(b) {
        borrow = sbb(borrow, *ai, *bi, bi);
    }

    borrow
}

fn sub2rev(a: &[BigDigit], b: &mut [BigDigit]) {
    debug_assert!(b.len() >= a.len());

    let len = Ord::min(a.len(), b.len());
    let (a_lo, a_hi) = a.split_at(len);
    let (b_lo, b_hi) = b.split_at_mut(len);

    let borrow = __sub2rev(a_lo, b_lo);

    assert!(a_hi.is_empty());

    // note: we're _required_ to fail on underflow
    assert!(
        borrow == 0 && b_hi.iter().all(|x| *x == 0),
        "Cannot subtract b from a because b is larger than a."
    );
}


impl<const N: usize> Sub<&BigUint<N>> for BigUint<N> {
    type Output = BigUint<N>;

    fn sub(self, other: &BigUint<N>) -> BigUint<N> {
        let mut res = self.clone();
        res -= other;
        res
    }
}

impl<const N: usize> SubAssign<&BigUint<N>> for BigUint<N> {
    fn sub_assign(&mut self, other: &BigUint<N>) {
        sub2(&mut self.0[..], &other.0[..]);
        // 移除高位0, 固定长度不需要这个操作
        // self.normalize();
    }
}


// impl Sub<u32> for BigUint {
//     type Output = BigUint;

//     #[inline]
//     fn sub(mut self, other: u32) -> BigUint {
//         self -= other;
//         self
//     }
// }

// impl SubAssign<u32> for BigUint {
//     fn sub_assign(&mut self, other: u32) {
//         sub2(&mut self.data[..], &[other as BigDigit]);
//         self.normalize();
//     }
// }

// impl Sub<BigUint> for u32 {
//     type Output = BigUint;

//     #[cfg(not(u64_digit))]
//     #[inline]
//     fn sub(self, mut other: BigUint) -> BigUint {
//         if other.data.len() == 0 {
//             other.data.push(self);
//         } else {
//             sub2rev(&[self], &mut other.data[..]);
//         }
//         other.normalized()
//     }

//     #[cfg(u64_digit)]
//     #[inline]
//     fn sub(self, mut other: BigUint) -> BigUint {
//         if other.data.is_empty() {
//             other.data.push(self as BigDigit);
//         } else {
//             sub2rev(&[self as BigDigit], &mut other.data[..]);
//         }
//         other.normalized()
//     }
// }

// impl Sub<u64> for BigUint {
//     type Output = BigUint;

//     #[inline]
//     fn sub(mut self, other: u64) -> BigUint {
//         self -= other;
//         self
//     }
// }

// impl SubAssign<u64> for BigUint {
//     #[cfg(not(u64_digit))]
//     #[inline]
//     fn sub_assign(&mut self, other: u64) {
//         let (hi, lo) = big_digit::from_doublebigdigit(other);
//         sub2(&mut self.data[..], &[lo, hi]);
//         self.normalize();
//     }

//     #[cfg(u64_digit)]
//     #[inline]
//     fn sub_assign(&mut self, other: u64) {
//         sub2(&mut self.data[..], &[other as BigDigit]);
//         self.normalize();
//     }
// }

// impl Sub<BigUint> for u64 {
//     type Output = BigUint;

//     #[cfg(not(u64_digit))]
//     #[inline]
//     fn sub(self, mut other: BigUint) -> BigUint {
//         while other.data.len() < 2 {
//             other.data.push(0);
//         }

//         let (hi, lo) = big_digit::from_doublebigdigit(self);
//         sub2rev(&[lo, hi], &mut other.data[..]);
//         other.normalized()
//     }

//     #[cfg(u64_digit)]
//     #[inline]
//     fn sub(self, mut other: BigUint) -> BigUint {
//         if other.data.is_empty() {
//             other.data.push(self);
//         } else {
//             sub2rev(&[self], &mut other.data[..]);
//         }
//         other.normalized()
//     }
// }


impl<const N: usize> BigUint<N> {
    #[inline]
    fn checked_sub(&self, v: &BigUint<N>) -> Option<Self> {
        let res = self.clone();
        match self.cmp(v) {
            Less => None,
            Equal => Some(Self::zero()),
            Greater => Some(res.sub(v)),
        }
    }
}
