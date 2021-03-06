use num_bigint::BigInt;
use num_bigint::Sign::Plus;
use num_traits::{Bounded, Signed, ToPrimitive, Zero};

use std::cmp::Ordering;
use std::ops::Neg;

mod consts;
use crate::consts::*;

#[macro_use]
mod macros;

#[test]
fn test_scalar_add() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x + y == z);
        assert_signed_scalar_assign_op!(x += y == z);
    }

    for elm in SUM_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&a, &b, &c);
        check(&b, &a, &c);
        check(&c, &na, &b);
        check(&c, &nb, &a);
        check(&a, &nc, &nb);
        check(&b, &nc, &na);
        check(&na, &nb, &nc);
        check(&a, &na, &Zero::zero());
    }
}

#[test]
fn test_scalar_sub() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x - y == z);
        assert_signed_scalar_assign_op!(x -= y == z);
    }

    for elm in SUM_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&c, &a, &b);
        check(&c, &b, &a);
        check(&nb, &a, &nc);
        check(&na, &b, &nc);
        check(&b, &na, &c);
        check(&a, &nb, &c);
        check(&nc, &na, &nb);
        check(&a, &a, &Zero::zero());
    }
}

#[test]
fn test_scalar_mul() {
    fn check(x: &BigInt, y: &BigInt, z: &BigInt) {
        let (x, y, z) = (x.clone(), y.clone(), z.clone());
        assert_signed_scalar_op!(x * y == z);
        assert_signed_scalar_assign_op!(x *= y == z);
    }

    for elm in MUL_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let (na, nb, nc) = (-&a, -&b, -&c);

        check(&a, &b, &c);
        check(&b, &a, &c);
        check(&na, &nb, &c);

        check(&na, &b, &nc);
        check(&nb, &a, &nc);
    }
}

#[test]
fn test_scalar_div_rem() {
    fn check_sub(a: &BigInt, b: u32, ans_q: &BigInt, ans_r: &BigInt) {
        let (q, r) = (a / b, a % b);
        if !r.is_zero() {
            assert_eq!(r.sign(), a.sign());
        }
        assert!(r.abs() <= b);
        assert!(*a == b * &q + &r);
        assert!(q == *ans_q);
        assert!(r == *ans_r);

        let b = BigInt::from(b);
        let (a, ans_q, ans_r) = (a.clone(), ans_q.clone(), ans_r.clone());
        assert_signed_scalar_op!(a / b == ans_q);
        assert_signed_scalar_op!(a % b == ans_r);
        assert_signed_scalar_assign_op!(a /= b == ans_q);
        assert_signed_scalar_assign_op!(a %= b == ans_r);

        let nb = -b;
        assert_signed_scalar_op!(a / nb == -ans_q.clone());
        assert_signed_scalar_op!(a % nb == ans_r);
        assert_signed_scalar_assign_op!(a /= nb == -ans_q.clone());
        assert_signed_scalar_assign_op!(a %= nb == ans_r);
    }

    fn check(a: &BigInt, b: u32, q: &BigInt, r: &BigInt) {
        check_sub(a, b, q, r);
        check_sub(&a.neg(), b, &q.neg(), &r.neg());
    }

    for elm in MUL_TRIPLES.iter() {
        let (a_vec, b_vec, c_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let b = BigInt::from_slice(Plus, b_vec);
        let c = BigInt::from_slice(Plus, c_vec);

        if a_vec.len() == 1 && a_vec[0] != 0 {
            let a = a_vec[0];
            check(&c, a, &b, &Zero::zero());
        }

        if b_vec.len() == 1 && b_vec[0] != 0 {
            let b = b_vec[0];
            check(&c, b, &a, &Zero::zero());
        }
    }

    for elm in DIV_REM_QUADRUPLES.iter() {
        let (a_vec, b_vec, c_vec, d_vec) = *elm;
        let a = BigInt::from_slice(Plus, a_vec);
        let c = BigInt::from_slice(Plus, c_vec);
        let d = BigInt::from_slice(Plus, d_vec);

        if b_vec.len() == 1 && b_vec[0] != 0 {
            let b = b_vec[0];
            check(&a, b, &c, &d);
        }
    }
}

#[test]
fn test_bigint_scalar_cmp() {
    let m_five = BigInt::from(-5);
    let m_one = BigInt::from(-1);
    let zero = BigInt::from(0);
    let one = BigInt::from(1);
    let five = BigInt::from(5);

    fn cmp_asserts(big: &BigInt, scalar: i32) {
        assert_eq!(big.partial_cmp(&(scalar as i8)), Some(Ordering::Equal));
        assert_eq!((scalar as i8).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as i8 - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!((scalar as i8 + 1).partial_cmp(big), Some(Ordering::Greater));
        assert_eq!(big.partial_cmp(&(scalar as i8 + 1)), Some(Ordering::Less));
        assert_eq!((scalar as i8 - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!(big.partial_cmp(&(scalar as i16)), Some(Ordering::Equal));
        assert_eq!((scalar as i16).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as i16 - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (scalar as i16 + 1).partial_cmp(big),
            Some(Ordering::Greater)
        );
        assert_eq!(big.partial_cmp(&(scalar as i16 + 1)), Some(Ordering::Less));
        assert_eq!((scalar as i16 - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!(big.partial_cmp(&(scalar as i32)), Some(Ordering::Equal));
        assert_eq!((scalar as i32).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as i32 - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (scalar as i32 + 1).partial_cmp(big),
            Some(Ordering::Greater)
        );
        assert_eq!(big.partial_cmp(&(scalar as i32 + 1)), Some(Ordering::Less));
        assert_eq!((scalar as i32 - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!(big.partial_cmp(&(scalar as i64)), Some(Ordering::Equal));
        assert_eq!((scalar as i64).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as i64 - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (scalar as i64 + 1).partial_cmp(big),
            Some(Ordering::Greater)
        );
        assert_eq!(big.partial_cmp(&(scalar as i64 + 1)), Some(Ordering::Less));
        assert_eq!((scalar as i64 - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!(big.partial_cmp(&(scalar as i128)), Some(Ordering::Equal));
        assert_eq!((scalar as i128).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as i128 - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (scalar as i128 + 1).partial_cmp(big),
            Some(Ordering::Greater)
        );
        assert_eq!(big.partial_cmp(&(scalar as i128 + 1)), Some(Ordering::Less));
        assert_eq!((scalar as i128 - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!(big.partial_cmp(&(scalar as isize)), Some(Ordering::Equal));
        assert_eq!((scalar as isize).partial_cmp(big), Some(Ordering::Equal));
        assert_eq!(
            big.partial_cmp(&(scalar as isize - 1)),
            Some(Ordering::Greater)
        );
        assert_eq!(
            (scalar as isize + 1).partial_cmp(big),
            Some(Ordering::Greater)
        );
        assert_eq!(
            big.partial_cmp(&(scalar as isize + 1)),
            Some(Ordering::Less)
        );
        assert_eq!((scalar as isize - 1).partial_cmp(big), Some(Ordering::Less));

        assert_eq!((scalar as i8), *big);
        assert_eq!(*big, (scalar as i16));
        assert_eq!((scalar as i16), *big);
        assert_eq!(*big, (scalar as i32));
        assert_eq!((scalar as i32), *big);
        assert_eq!(*big, (scalar as i64));
        assert_eq!((scalar as i64), *big);
        assert_eq!(*big, (scalar as i128));
        assert_eq!((scalar as i128), *big);
        assert_eq!(*big, (scalar as isize));
        assert_eq!((scalar as isize), *big);

        if scalar > 0 {
            assert_eq!(*big, (scalar as u8));
            assert_eq!((scalar as u8), *big);
            assert_eq!(*big, (scalar as u16));
            assert_eq!((scalar as u16), *big);
            assert_eq!(*big, (scalar as u32));
            assert_eq!((scalar as u32), *big);
            assert_eq!(*big, (scalar as u64));
            assert_eq!((scalar as u64), *big);
            assert_eq!(*big, (scalar as u128));
            assert_eq!((scalar as u128), *big);
            assert_eq!(*big, (scalar as usize));
            assert_eq!((scalar as usize), *big);
        }
    }

    cmp_asserts(&zero, 0i32);
    cmp_asserts(&one, 1i32);
    cmp_asserts(&five, 5i32);
    cmp_asserts(&m_five, -5i32);
    cmp_asserts(&m_one, -1i32);
}

#[test]
fn test_bigint_scalar_cmp_limits() {
    fn check<T>()
    where
        T: Copy + Bounded,
        BigInt: From<T> + PartialOrd<T>,
    {
        let min = T::min_value();
        let big_min = BigInt::from(min);
        assert_eq!(big_min.partial_cmp(&min), Some(Ordering::Equal));
        assert_eq!((&big_min - 1u32).partial_cmp(&min), Some(Ordering::Less));
        assert_eq!((&big_min + 1u32).partial_cmp(&min), Some(Ordering::Greater));

        let max = T::max_value();
        let big_max = BigInt::from(max);
        assert_eq!(big_max.partial_cmp(&max), Some(Ordering::Equal));
        assert_eq!((&big_max - 1u32).partial_cmp(&max), Some(Ordering::Less));
        assert_eq!((&big_max + 1u32).partial_cmp(&max), Some(Ordering::Greater));
    }

    check::<i8>();
    check::<i16>();
    check::<i32>();
    check::<i64>();
    check::<i128>();
    check::<isize>();

    check::<u8>();
    check::<u16>();
    check::<u32>();
    check::<u64>();
    check::<u128>();
    check::<usize>();
}
