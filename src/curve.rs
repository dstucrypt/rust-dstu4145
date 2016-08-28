extern crate num;

use num::{BigUint, One, Zero};
use std::ops::{BitAnd, Shl};

use gf2m;

pub fn infinity()-> (BigUint, BigUint) {
    return (BigUint::zero(), BigUint::zero());
}

pub fn at_infinity(value_x: &BigUint, value_y: &BigUint) -> bool {
    return value_x.is_zero() && value_y.is_zero();
}

pub fn point_add(value_ax: &BigUint, value_ay: &BigUint,
                     value_bx: &BigUint, value_by: &BigUint,
                     modulus: &BigUint,
                     curve_a: &BigUint) -> (BigUint, BigUint) {

    if at_infinity(value_ax, value_ay) {
        return (value_bx.clone(), value_by.clone());
    }

    if at_infinity(value_bx, value_by) {
        return (value_ax.clone(), value_ay.clone());
    }

    let lbd;
    let value_cx;

    if value_ax.eq(value_bx) == false {
        let neg_abx = gf2m::neg(gf2m::add(value_ax, value_bx), modulus);
        lbd = gf2m::fmod(
            gf2m::mul(
                &gf2m::add(value_ay, value_by),
                &neg_abx,
            ),
            modulus
        );
        let temp_cx = gf2m::add(
            curve_a,
            &gf2m::fmod(gf2m::mul(&lbd, &lbd), modulus)
        );
        let temp_cx = gf2m::add(&temp_cx, &lbd);
        let temp_cx = gf2m::add(&temp_cx, value_ax);
        value_cx = gf2m::add(&temp_cx, value_bx);
    }
    else if value_ay.eq(value_by) == false {
        return infinity();
    }
    else if value_ax.is_zero() {
        return infinity();
    }
    else {
        let neg_ax = gf2m::neg(value_ax.clone(), modulus);
        lbd = gf2m::add(
            value_ax,
            &gf2m::fmod(
                gf2m::mul(value_ay, &neg_ax),
                modulus
            )
        );
        let temp = gf2m::add(
            curve_a,
            &gf2m::fmod(
                gf2m::mul(&lbd, &lbd),
                modulus
            )
        );
        value_cx = gf2m::add(&temp, &lbd);;
    }

    let value_cy = gf2m::fmod(
        gf2m::mul(&gf2m::add(value_bx, &value_cx), &lbd),
        modulus
    );
    let value_cy = gf2m::add(&value_cy, &value_cx);
    let value_cy = gf2m::add(&value_cy, value_by);

    return (value_cx, value_cy);
}

// FIXME: negative mul impossible
pub fn point_mul(value_ax: &BigUint, value_ay: &BigUint,
                     factor: &BigUint,
                     modulus: &BigUint,
                     curve_a: &BigUint) -> (BigUint, BigUint) {

    if factor.is_zero() {
        return (BigUint::zero(), BigUint::zero());
    }

    let mut value_x = BigUint::zero();
    let mut value_y = BigUint::zero();
	let mut j = factor.bits() as i32;

    while j >= 0 {
        let (temp_x, temp_y) = point_add(
            &value_x, &value_y, &value_x, &value_y,
            modulus, curve_a
        );
        value_x = temp_x;
        value_y = temp_y;

        let mask = BigUint::one().shl(j as usize);
        let test = factor.bitand(mask);
        if test.is_zero() == false {
            let (temp_x, temp_y) = point_add(
                &value_ax, &value_ay, &value_x, &value_y,
                modulus, curve_a
            );
            value_x = temp_x;
            value_y = temp_y;
        }
        j = j - 1;
    }

    return (value_x, value_y);
}
