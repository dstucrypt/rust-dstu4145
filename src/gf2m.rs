extern crate num;

use num::{BigUint, One, Zero};
use std::ops::{BitOr, BitAnd, BitXor, Shl};
use std::cmp::Ordering::{Less};


pub fn compute_modulus(p1: usize, k1: usize, k2: usize, k3: usize) -> BigUint {
    let one =  BigUint::one();
    let modulus = one.shl(p1);
    let modulus = modulus.bitor(BigUint::one().shl(k1));
    let modulus = modulus.bitor(BigUint::one().shl(k2));
    let modulus = modulus.bitor(BigUint::one().shl(k3));

    return modulus;

}

pub fn fmod(value: BigUint, modulus: &BigUint) -> BigUint {
	let mut value = match value.cmp(modulus) {
		Less => return value,
        _ => value,
	};
    while value.bits() >= modulus.bits() {
        let mask = modulus.shl(value.bits() - modulus.bits());
        value = value.bitxor(&mask);
    }
    return value;
}

pub fn mul(value_a: &BigUint, value_b: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut j = 0;
    let mut temp_b = value_b.clone();
    while j < value_a.bits() {
        let mask = BigUint::one().shl(j);
        let test = value_a.bitand(mask);
        if test.is_zero() == false {
            result = result.bitxor(&temp_b);
        }
        temp_b = temp_b.shl(1);
        j = j + 1;
    }
    return result;
}

pub fn neg(value: BigUint, modulus: &BigUint) -> BigUint {
    let mut b = BigUint::one();
    let mut c = BigUint::zero();
    let mut u = fmod(value, modulus);
    let mut v = modulus.clone();

    while u.bits() > 1 {
        let mut j: i32 = (u.bits() as i32) - (v.bits() as i32);

        if j < 0 {
            let temp = u;
            u = v;
            v = temp;

            let temp = c;
            c = b;
            b = temp;

            j = -j;
        }
        let ref vref = v;
        let ref cref = c;
        u = add(&u, &vref.shl(j as usize));
        b = add(&b, &cref.shl(j as usize));
    }

    return b;
}


pub fn add(value_a: &BigUint, value_b: &BigUint)-> BigUint {
    return value_a.bitxor(value_b);
}

pub fn truncate(value: &BigUint, size: usize) -> BigUint {
    let one = &BigUint::one();
    let mut result = value.clone();
    while size <= result.bits() {
        result = add(&result, &one.shl(result.bits() - 1));
    }
    return result;
}
