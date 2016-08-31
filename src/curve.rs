extern crate num;

use num::{BigUint, One, Zero};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Sub};

use gf2m;

#[derive(Clone, Debug, Hash)]
pub struct Point {
    pub x: BigUint,
    pub y: BigUint,
}

impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        return self.x.eq(&other.x) && self.y.eq(&other.y);
    }
}

pub struct Curve {
    pub param_a: BigUint,
    pub param_b: BigUint,
    pub order: BigUint,
    pub base: Point,
    pub field_m: usize,
    pub field_k1: usize,
    pub field_k2: usize,
    pub field_k3: usize,
    pub modulus: BigUint,
}

pub fn infinity()-> Point {
    return Point {x: BigUint::zero(), y: BigUint::zero()};
}

pub fn at_infinity(value_x: &BigUint, value_y: &BigUint) -> bool {
    return value_x.is_zero() && value_y.is_zero();
}

pub fn point_add(point_a: &Point, point_b: &Point,
                 modulus: &BigUint,
                 curve_a: &BigUint) -> Point {

    if at_infinity(&point_a.x, &point_a.y) {
        return point_b.clone();
    }

    if at_infinity(&point_b.x, &point_b.y) {
        return point_a.clone();
    }

    let lbd;
    let value_cx;

    if point_a.x.eq(&point_b.x) == false {
        let neg_abx = gf2m::neg(gf2m::add(&point_a.x, &point_b.x), modulus);
        lbd = gf2m::fmod(
            gf2m::mul(
                &gf2m::add(&point_a.y, &point_b.y),
                &neg_abx,
            ),
            modulus
        );
        let temp_cx = gf2m::add(
            curve_a,
            &gf2m::fmod(gf2m::mul(&lbd, &lbd), modulus)
        );
        let temp_cx = gf2m::add(&temp_cx, &lbd);
        let temp_cx = gf2m::add(&temp_cx, &point_a.x);
        value_cx = gf2m::add(&temp_cx, &point_b.x);
    }
    else if point_a.y.eq(&point_b.y) == false {
        return infinity();
    }
    else if point_a.x.is_zero() {
        return infinity();
    }
    else {
        let neg_ax = gf2m::neg(point_a.x.clone(), modulus);
        lbd = gf2m::add(
            &point_a.x,
            &gf2m::fmod(
                gf2m::mul(&point_a.y, &neg_ax),
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
        gf2m::mul(&gf2m::add(&point_b.x, &value_cx), &lbd),
        modulus
    );
    let value_cy = gf2m::add(&value_cy, &value_cx);
    let value_cy = gf2m::add(&value_cy, &point_b.y);

    return Point {x: value_cx, y: value_cy};
}

// FIXME: negative mul impossible
pub fn point_mul(point: &Point, factor: &BigUint,
                 modulus: &BigUint,
                 curve_a: &BigUint) -> Point {

    if factor.is_zero() {
        return infinity();
    }

    let mut value = infinity();
	let mut j = factor.bits() as i32;

    while j >= 0 {
        value = point_add(
            &value, &value,
            modulus, curve_a
        );

        let mask = BigUint::one().shl(j as usize);
        let test = factor.bitand(mask);
        if test.is_zero() == false {
            value = point_add(
                point, &value,
                modulus, curve_a
            );
        }
        j = j - 1;
    }

    return value;
}

pub fn point_expand(compressed: &BigUint, curve: &Curve)-> Point {

    let mut value = compressed.clone();
    if compressed.is_zero() {
        let mulpb = gf2m::fmod(
            gf2m::mul(&curve.param_b, &curve.param_b),
            &curve.modulus
        );
        return Point {x: value, y: mulpb}
    }
    let k = BigUint::one().bitand(&value);

    let mask = BigUint::one().shl(curve.field_m).sub(BigUint::from(2 as u8));
    value = value.bitand(mask);

    let trace = gf2m::trace(&value, &curve.modulus);
    if (trace == 1 && curve.param_a.is_zero()) ||
       (trace == 0 && !curve.param_a.is_zero()) {
        value = value.bitor(BigUint::one());
    }
    let x2 = gf2m::fmod(gf2m::mul(&value, &value), &curve.modulus);
    let mut y = gf2m::fmod(gf2m::mul(&x2, &value), &curve.modulus);

    if !curve.param_a.is_zero() {
        y = gf2m::add(&y, &x2);
    }

    y = gf2m::add(&y, &curve.param_b);
    let invx2 = gf2m::neg(x2, &curve.modulus);
    y = gf2m::fmod(gf2m::mul(&y, &invx2), &curve.modulus);

    y = gf2m::fmod(
        gf2m::squad_odd(&y, &curve.modulus, curve.field_m),
        &curve.modulus
    );

    let trace_y = gf2m::trace(&y, &curve.modulus);

    if (!k.is_zero() && trace_y == 0) ||
       (k.is_zero() && trace_y == 1) {
        y = y.bitxor(BigUint::one());
    }

    y = gf2m::fmod(gf2m::mul(&y, &value), &curve.modulus);

    return Point {x: value, y: y};
}
