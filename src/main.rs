extern crate num;

use num::{BigUint, Zero};

mod gf2m;
mod curve;
mod dstu4145;
mod dstu_params;

use curve::Point;


fn test_dstu4145_sign_helper() {
    let curve = dstu_params::curve_257();

    let priv_d = big(b"2A45EAFE4CD469F811737780C57253360FBCC58E134C9A1FDCD10B0E4529A143");
	let to_be_signed = big(b"6845214B63288A832A772E1FE6CB6C7D3528569E29A8B3584370FDC65F474242");
	let rand_e = big(b"7A32849E569C8888F25DE6F69A839D75057383F473ACF559ABD3C5D683294CEB");

    assert_eq!(
        dstu4145::sign_helper(&priv_d, &to_be_signed, rand_e, &curve).unwrap(),
        (
            big(b"0CCC6816453A903A1B641DF999011177DF420D21A72236D798532AEF42E224AB"),
            big(b"491FA1EF75EAEF75E1F20CF3918993AB37E06005EA8E204BC009A1FA61BB0FB2")
        )
    );
}

fn test_dstu4145_verify_helper() {
    let curve = dstu_params::curve_257();

    let s = big(b"0CCC6816453A903A1B641DF999011177DF420D21A72236D798532AEF42E224AB");
    let r = big(b"491FA1EF75EAEF75E1F20CF3918993AB37E06005EA8E204BC009A1FA61BB0FB2");
	let to_be_signed = big(b"6845214B63288A832A772E1FE6CB6C7D3528569E29A8B3584370FDC65F474242");

    let pubkey = Point {
        x: big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a6589"),
        y: big(b"1b345bc134f27da251edfae97b3f306b4e8b8cb9cf86d8651e4fb301ef8e1239c")
    };

    assert_eq!(
        dstu4145::verify_helper(
            &pubkey,
            &s, &r, &to_be_signed,
            &curve
        ),
        true
    );
}

fn big(bytes: &[u8])-> BigUint {
    return BigUint::parse_bytes(bytes, 16).unwrap();
}

fn test_compute_modulus () {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    assert_eq!(mod257, big(b"20000000000000000000000000000000000000000000000000000000000001001"));
}

fn test_field_mod() {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    assert_eq!(
        gf2m::fmod(value_a, &mod257),
        big(b"ff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a60895")
    );
}

fn test_field_mul() {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
    let value_b = big(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

    assert_eq!(
        gf2m::fmod(gf2m::mul(&value_a, &value_b), &mod257),
        big(b"beb7d8390bb24fcf6882086cddd4ebe5270c1ed345bc516b40efb92b44530d5f")
    );

}

fn test_field_neg() {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let value_a = big(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

    assert_eq!(
        gf2m::neg(value_a, &mod257),
        big(b"f5ae84d0c4dc2e7e89c670fb2083d124be50b413efb6863705bd63a5168352e0")
    );
}

fn test_point_double() {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();
    let point = Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    assert_eq!(
        curve::point_add(&point, &point, &mod257, &curve_a),
        Point {
            x: big(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
            y: big(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
        }
    );
}

fn test_point_mul() {
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
    let curve_a = BigUint::zero();
    let privd = big(b"2A45EAFE4CD469F811737780C57253360FBCC58E134C9A1FDCD10B0E4529A143");

    let point = Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    assert_eq!(
        curve::point_mul(&point, &privd, &mod257, &curve_a),
        Point {
            x: big(b"8c3d388b1c51116cf0ed041718309b360f775d8df86e9fc141822e79a3b0da8b"),
            y: big(b"a8624188d9f4ab0afafbde6230cd8cf7c28b38f42fcbb4021ff0c0244a5ddbbd")
        }
    );
}


fn test_point_add() {
    let curve_a = BigUint::zero();
    let mod257 = gf2m::compute_modulus(257, 12, 0, 0);

    let point = Point {
        x: big(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
        y: big(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
    };

    let point_2 = Point {
        x: big(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
        y: big(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
    };

    assert_eq!(
        curve::point_add(&point, &point_2, &mod257, &curve_a),
        Point {
            x: big(b"9a826cff814626da47bc409383d83922f65ec3e890e3b41a60e89f3a864c2766"),
            y: big(b"1e465ea7610428ec6b0b56be039dd73f3fe18d7d7731d60a18ff9224caaf43b76")
        }
    );

}

fn main () {
    test_compute_modulus();
    test_field_mod();
    test_field_mul();
    test_field_neg();

    test_point_double();
    test_point_add();
    test_point_mul();

    test_dstu4145_sign_helper();
    test_dstu4145_verify_helper();
}
