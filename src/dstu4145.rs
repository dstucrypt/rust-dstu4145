/*
use num::{BigUint, Zero};
use std::ops::{Add, Mul, Rem};
use std::option::Option;
use std::cmp::Ordering::{Greater}; */

use curve;
use curve::{Point, Curve};
use gf2m;
use gf2m::Field;

/*
pub fn sign_helper(priv_d: &BigUint, tbs: &BigUint, rand_e: BigUint,
                   curve: &Curve) -> Option<(BigUint, BigUint)> {

    let point_g = curve::point_mul(
        &curve.base, &rand_e,
        &curve.modulus, &curve.param_a
    );

    if point_g.x.is_zero() {
        return None;
    }

    let tbs = gf2m::truncate(tbs, curve.field_m);
    let r = gf2m::fmod(gf2m::mul(&tbs, &point_g.x), &curve.modulus);
    let r = gf2m::truncate(&r, curve.order.bits());
    
    if r.is_zero() {
        return None;
    }

    let s = priv_d.mul(&r).rem(&curve.order);
    let s = s.add(rand_e).rem(&curve.order);

    return Some((s, r));
} */

/*
fn gt(lft: &BigUint, rgt: &BigUint) -> bool {
    return match lft.cmp(rgt) {
        Greater => true,
        _  => false,
    };
} */

pub fn verify_helper(public: &Point,
                     param_s: &Field, param_r: &Field,
                     tbs: &Field,
                     curve: &Curve) -> bool {
    if gf2m::is_zero(param_s) {
        return false;
    }

    if gf2m::is_zero(param_r) {
        return false;
    }

    /*
    if gt(param_s, &curve.order) {
        return false;
    } */

    let point_mulq = curve::point_mul(public, &param_r, &curve.modulus, &curve.param_a);
    let point_muls = curve::point_mul(&curve.base, &param_s, &curve.modulus, &curve.param_a);

    let point_r = curve::point_add(&point_mulq, &point_muls, &curve.modulus, &curve.param_a);

    if curve::at_infinity(&point_r.x, &point_r.y) {
        return false;
    }

    let compare_r = gf2m::reduce(
        &gf2m::mul(tbs, &point_r.x),
        &curve.modulus
    );
    let compare_r = gf2m::truncate(&compare_r, gf2m::bit_size(&curve.order));

    return compare_r.eq(param_r);
}
