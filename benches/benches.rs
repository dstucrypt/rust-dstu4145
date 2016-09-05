#![feature(test)]
extern crate test;
extern crate dstu4145;

use test::Bencher;

use dstu4145::gf2m;
use dstu4145::curve;
use dstu4145::dstu_params;


#[bench]
fn bench_add(b: &mut Bencher) {
    let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = gf2m::parse_hex(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    b.iter(|| {
        gf2m::add(&value_a, &value_b);
    });
}


#[bench]
fn bench_reduce(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = gf2m::parse_hex(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    let long_value = gf2m::mul(&value_a, &value_b);

    b.iter(|| {
        gf2m::reduce(&long_value, &mod257);
    });
}


#[bench]
fn bench_reduce_2(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    b.iter(|| {
        gf2m::reduce(&value_a, &mod257);
    });
}

#[bench]
fn bench_mul(b: &mut Bencher) {
    let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = gf2m::parse_hex(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    b.iter(|| {
        gf2m::mul(&value_a, &value_b);
    });
}

#[bench]
fn bench_neg(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    b.iter(|| {
        gf2m::neg(&value_a, &mod257);
    });
}

#[bench]
fn bench_point_double(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = gf2m::zero();
    let point = curve::Point {
        x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_dbl(&point, &mod257, &curve_a);
    });
}

#[bench]
fn bench_point_add(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = gf2m::zero();

    let point = curve::Point {
        x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    let point_2 = curve::Point {
        x: gf2m::parse_hex(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
        y: gf2m::parse_hex(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
    };


    b.iter(|| {
        curve::point_add(&point, &point_2, &mod257, &curve_a);
    });
}


#[bench]
fn bench_point_add_same(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = gf2m::zero();

    let point = curve::Point {
        x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_add(&point, &point, &mod257, &curve_a);
    });
}


#[bench]
fn bench_point_mul(b: &mut Bencher) {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = gf2m::zero();
    let privd = gf2m::parse_hex(b"2A45EAFE4CD469F811737780C57253360FBCC58E134C9A1FDCD10B0E4529A143");

    let point = curve::Point {
        x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    b.iter(|| {
        curve::point_mul(&point, &privd, &mod257, &curve_a);
    });
}

#[bench]
fn bench_verify_sign(b: &mut Bencher) {
    let curve = dstu_params::curve_257();

    let s = gf2m::parse_hex(b"0CCC6816453A903A1B641DF999011177DF420D21A72236D798532AEF42E224AB");
    let r = gf2m::parse_hex(b"491FA1EF75EAEF75E1F20CF3918993AB37E06005EA8E204BC009A1FA61BB0FB2");
    let to_be_signed = gf2m::parse_hex(b"6845214B63288A832A772E1FE6CB6C7D3528569E29A8B3584370FDC65F474242");

    let pubkey = curve::Point {
        x: gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a6589"),
        y: gf2m::parse_hex(b"1b345bc134f27da251edfae97b3f306b4e8b8cb9cf86d8651e4fb301ef8e1239c")
    };

    b.iter(|| {
        dstu4145::verify_helper(
            &pubkey,
            &s, &r, &to_be_signed,
            &curve
        );
    });
}
