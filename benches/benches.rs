#![feature(test)]
extern crate test;
extern crate dstu4145;
extern crate num;

use num::{BigUint, Zero};

use test::Bencher;

use dstu4145::gf2m;
use dstu4145::curve;
use dstu4145::dstu_params;

fn big(bytes: &[u8])-> BigUint {
    return BigUint::parse_bytes(bytes, 16).unwrap();
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = big(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    b.iter(|| {
        gf2m::add(&value_a, &value_b);
    });
}

#[bench]
fn bench_reduce(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = big(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    let long_value = gf2m::mul(&value_a, &value_b);

    b.iter(|| {
        gf2m::fmod(long_value.clone(), &mod257);
    });
}


#[bench]
fn bench_reduce_2(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    b.iter(|| {
        gf2m::fmod(value_a.clone(), &mod257);
    });
}

#[bench]
fn bench_mul(b: &mut Bencher) {
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = big(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    b.iter(|| {
        gf2m::mul(&value_a, &value_b);
    });
}

#[bench]
fn bench_neg(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    b.iter(|| {
        gf2m::neg(value_a.clone(), &mod257);
    });
}

#[bench]
fn bench_point_double(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();
    let point = curve::Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_dbl(&point, &mod257, &curve_a);
    });
}

#[bench]
fn bench_point_add(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();

    let point = curve::Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    let point_2 = curve::Point {
        x: big(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
        y: big(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
    };


    b.iter(|| {
        curve::point_add(&point, &point_2, &mod257, &curve_a);
    });
}


#[bench]
fn bench_point_add_same(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();

    let point = curve::Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_add(&point, &point, &mod257, &curve_a);
    });
}

#[bench]
#[ignore]
fn bench_point_mul(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();
    let privd = big(b"2A45EAFE4CD469F811737780C57253360FBCC58E134C9A1FDCD10B0E4529A143");

    let point = curve::Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_mul(&point, &privd, &mod257, &curve_a);
    });
}

#[bench]
#[ignore]
fn bench_point_expand_431(b: &mut Bencher) {
    let curve = dstu_params::curve_431();
    let point_data: [u8; 54] = [
        0xb6, 0x1b, 0xf9, 0xbd, 0x4b, 0x62, 0xca, 0xab,
        0x2c, 0x39, 0x50, 0xf5, 0xc5, 0x1d, 0x5f, 0xa8,
        0x0d, 0x70, 0x7e, 0x00, 0x7b, 0x52, 0x5b, 0x70,
        0x67, 0x67, 0xdc, 0xe5, 0xcd, 0x1b, 0xaf, 0x6e,
        0x27, 0x68, 0xda, 0xd0, 0xc6, 0xa8, 0x4f, 0xc2,
        0x2f, 0x99, 0x05, 0x1d, 0x91, 0x34, 0x35, 0xf4,
        0xeb, 0x1e, 0xb1, 0x9a, 0xd5, 0x44
    ];

    let point_compressed_x = BigUint::from_bytes_le(&point_data);

    b.iter(|| {
        curve::point_expand(&point_compressed_x, &curve);
    });
}


