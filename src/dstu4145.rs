use num::{BigUint, Zero};
use std::ops::{Add, Mul, Rem};
use std::option::Option;
use std::cmp::Ordering::{Greater};

use curve;
use gf2m;

pub fn sign_helper(priv_d: &BigUint, tbs: &BigUint, rand_e: BigUint,
                        base_x: &BigUint, base_y: &BigUint,
                        order: &BigUint, param_m: usize,
                        modulus: &BigUint, curve_a: &BigUint)-> Option<(BigUint, BigUint)> {

    let (pointg_x, _pointg_y) = curve::point_mul(base_x, base_y, &rand_e, modulus, curve_a);

    if pointg_x.is_zero() {
        return None;
    }

    let tbs = gf2m::truncate(tbs, param_m);
    let r = gf2m::fmod(gf2m::mul(&tbs, &pointg_x), modulus);
    let r = gf2m::truncate(&r, order.bits());
    
    if r.is_zero() {
        return None;
    }

    let s = priv_d.mul(&r).rem(order);
    let s = s.add(rand_e).rem(order);

    return Some((s, r));
}

fn gt(lft: &BigUint, rgt: &BigUint) -> bool {
    return match lft.cmp(rgt) {
        Greater => true,
        _  => false,
    };
}

pub fn verify_helper(public_x: &BigUint, public_y: &BigUint,
                     param_s: &BigUint, param_r: &BigUint,
                     tbs: &BigUint,
                     base_x: &BigUint, base_y: &BigUint,
                     order: &BigUint,
                     modulus: &BigUint, curve_a: &BigUint) -> bool {
    if param_s.is_zero() {
        return false;
    }

    if param_r.is_zero() {
        return false;
    }

    if gt(param_s, order) {
        return false;
    }

    let (point_mulq_x, point_mulq_y) = curve::point_mul(public_x, public_y, &param_r, modulus, curve_a);
    let (point_muls_x, point_muls_y) = curve::point_mul(base_x, base_y, &param_s, modulus, curve_a);

    let (pointr_x, pointr_y) = curve::point_add(&point_mulq_x, &point_mulq_y, &point_muls_x, &point_muls_y, modulus, curve_a);

    if curve::at_infinity(&pointr_x, &pointr_y) {
        return false;
    }

    let compare_r = gf2m::fmod(
        gf2m::mul(tbs, &pointr_x),
        modulus
    );
    let compare_r = gf2m::truncate(&compare_r, order.bits());

    return compare_r.eq(param_r);
}
