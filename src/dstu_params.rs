use num::{BigUint, One, Zero};

use gf2m;
use curve::{Curve, Point};
use curve2;


pub fn curve_257() -> Curve {
    return Curve {
        param_a: BigUint::zero(),
        param_b: BigUint::parse_bytes(b"01CEF494720115657E18F938D7A7942394FF9425C1458C57861F9EEA6ADBE3BE10", 16).unwrap(),
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

pub fn curve_431() -> Curve {
    return Curve {
        param_a: BigUint::one(),
        param_b: BigUint::parse_bytes(b"03CE10490F6A708FC26DFE8C3D27C4F94E690134D5BFF988D8D28AAEAEDE975936C66BAC536B18AE2DC312CA493117DAA469C640CAF3", 16).unwrap(),
        order: BigUint::parse_bytes(b"3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFBA3175458009A8C0A724F02F81AA8A1FCBAF80D90C7A95110504CF", 16).unwrap(),
        base: Point {
            x: BigUint::parse_bytes(b"1A62BA79D98133A16BBAE7ED9A8E03C32E0824D57AEF72F88986874E5AAE49C27BED49A2A95058068426C2171E99FD3B43C5947C857D", 16).unwrap(),
            y: BigUint::parse_bytes(b"70B5E1E14031C1F70BBEFE96BDDE66F451754B4CA5F48DA241F331AA396B8D1839A855C1769B1EA14BA53308B5E2723724E090E02DB9", 16).unwrap(),
        },
        field_m: 431,
        field_k1: 5,
        field_k2: 3,
        field_k3: 1,
        modulus: gf2m::compute_modulus(431, 5, 3, 1),
    };
}

pub fn curve_431_bytes() -> curve2::Curve {
    return curve2::Curve {
        param_a: gf2m::one(),
        param_b: gf2m::parse_hex(b"03CE10490F6A708FC26DFE8C3D27C4F94E690134D5BFF988D8D28AAEAEDE975936C66BAC536B18AE2DC312CA493117DAA469C640CAF3"),
        order: gf2m::parse_hex(b"3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFBA3175458009A8C0A724F02F81AA8A1FCBAF80D90C7A95110504CF"),
        base: curve2::Point {
            x: gf2m::parse_hex(b"1A62BA79D98133A16BBAE7ED9A8E03C32E0824D57AEF72F88986874E5AAE49C27BED49A2A95058068426C2171E99FD3B43C5947C857D"),
            y: gf2m::parse_hex(b"70B5E1E14031C1F70BBEFE96BDDE66F451754B4CA5F48DA241F331AA396B8D1839A855C1769B1EA14BA53308B5E2723724E090E02DB9"),
        },
        field_m: 431,
        field_k1: 5,
        field_k2: 3,
        field_k3: 1,
        modulus: gf2m::compute_modulus_bytes(431, 5, 3, 1),
    };
}
