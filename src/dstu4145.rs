use num::{BigUint, Zero};
use std::ops::{Add, Mul, Rem};
use std::option::Option;

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

