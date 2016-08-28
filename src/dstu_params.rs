use num::{BigUint, Zero};

use gf2m;
use curve::{Curve, Point};


pub fn curve_257() -> Curve {
    return Curve {
        param_a: BigUint::zero(),
        order: BigUint::parse_bytes(b"800000000000000000000000000000006759213af182e987d3e17714907d470d", 16).unwrap(),
        base: Point {
            x: BigUint::parse_bytes(b"002A29EF207D0E9B6C55CD260B306C7E007AC491CA1B10C62334A9E8DCD8D20FB7", 16).unwrap(),
            y: BigUint::parse_bytes(b"010686D41FF744D4449FCCF6D8EEA03102E6812C93A9D60B978B702CF156D814EF", 16).unwrap()
        },
        field_m: 257,
        field_k1: 12,
        field_k2: 0,
        field_k3: 0,
        modulus: gf2m::compute_modulus(257, 12, 0, 0),
    };
}
