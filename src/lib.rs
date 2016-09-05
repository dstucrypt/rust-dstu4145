pub mod gf2m;
pub mod curve;
pub mod dstu4145;
pub mod dstu_params;

pub use dstu4145::verify_helper;

#[cfg(test)]
mod test {
    use curve::Point;
    use super::dstu_params;
    use super::dstu4145;
    use super::gf2m;
    use super::curve;

    /*
    #[test]
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
    } */

    #[test]
    fn test_dstu4145_verify_helper() {
        let curve = dstu_params::curve_257();

        let s = gf2m::parse_hex(b"0CCC6816453A903A1B641DF999011177DF420D21A72236D798532AEF42E224AB");
        let r = gf2m::parse_hex(b"491FA1EF75EAEF75E1F20CF3918993AB37E06005EA8E204BC009A1FA61BB0FB2");
        let to_be_signed = gf2m::parse_hex(b"6845214B63288A832A772E1FE6CB6C7D3528569E29A8B3584370FDC65F474242");

        let pubkey = Point {
            x: gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a6589"),
            y: gf2m::parse_hex(b"1b345bc134f27da251edfae97b3f306b4e8b8cb9cf86d8651e4fb301ef8e1239c")
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


    #[test]
    fn test_compute_modulus () {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        assert_eq!(mod257, gf2m::parse_hex(b"20000000000000000000000000000000000000000000000000000000000001001"));
        let mod431 = gf2m::compute_modulus(431, 5, 3, 1);
        assert_eq!(mod431, gf2m::parse_hex(b"80000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002b"));
    }

    #[test]
    fn test_field_add() {
        let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
        let value_b = gf2m::parse_hex(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");
        let sum = gf2m::add(&value_a, &value_b);

        assert_eq!(sum, gf2m::parse_hex(b"ccaf166ff5ff0fe2100b5a02e0bfc5ffa7c6ee894ac6dfaf446b76d54a56945b"));
    }

    #[test]
    fn test_field_mod() {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

        assert_eq!(
            gf2m::reduce(&value_a, &mod257),
            gf2m::parse_hex(b"ff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a60895")
        );
    }

    #[test]
    fn test_field_shl() {
        let value = [0x92468ACD; 32];
        let shifted = gf2m::shll(&value, 8);
        let mut expect = [
            0x468ACD92; 32
        ];
        expect[0] = 0x468ACD00;
        assert_eq!(shifted, expect);
    }

    #[test]
    fn test_field_shl_word() {
        let value = [0x92468ACD; 32];
        let shifted = gf2m::shll(&value, 132);

        let mut expect = [
            0x2468ACD9; 32
        ];
        expect[4] = 0x2468ACD0;
        expect[3] = 0;
        expect[2] = 0;
        expect[1] = 0;
        expect[0] = 0;

        assert_eq!(shifted, expect);
    }

    #[test]
    fn test_field_mul() {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");
        let value_b = gf2m::parse_hex(b"a3391f6f341d627ab958fc4223ee8871e336c8d9dda30f407c369268363f0cccb");

        assert_eq!(
            gf2m::reduce(&gf2m::mul(&value_a, &value_b), &mod257),
            gf2m::parse_hex(b"beb7d8390bb24fcf6882086cddd4ebe5270c1ed345bc516b40efb92b44530d5f")
        );
    }

    #[test]
    fn test_field_neg() {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        let value_a = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

        assert_eq!(
            gf2m::neg(&value_a, &mod257),
            gf2m::parse_hex(b"f5ae84d0c4dc2e7e89c670fb2083d124be50b413efb6863705bd63a5168352e0")
        );
    }

    #[test]
    fn test_field_parsehex() {
        let value_a: gf2m::Field = [
            698767504, 2265075798, 2432052136, 2494194452, 3730260705, 2240060960, 3022596169, 4282310812, 10, 0, 0, 0, 0, 0, 0, 0
        ];
        let expect = gf2m::parse_hex(b"aff3ee09cb429284985849e20de5742e194aa631490f62ba88702505629a65890");

        assert_eq!(value_a, expect);
    }

    #[test]
    fn test_point_double() {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        let curve_a = gf2m::zero();
        let point = Point {
            x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
            y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
        };

        assert_eq!(
            curve::point_dbl(&point, &mod257, &curve_a),
            Point {
                x: gf2m::parse_hex(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
                y: gf2m::parse_hex(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
            }
        );
    }

    #[test]
    fn test_point_add() {
        let curve_a = gf2m::zero();
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);

        let point = Point {
            x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
            y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
        };

        let point_2 = Point {
            x: gf2m::parse_hex(b"176dbde19773dfd335665597e8d6a0ab678721a5bb7030f25dc4c48b809ef3520"),
            y: gf2m::parse_hex(b"6e75301556ea5d571403086691030f024c026907c8e818b2eedd9184d12040ee")
        };

        assert_eq!(
            curve::point_add(&point, &point_2, &mod257, &curve_a),
            Point {
                x: gf2m::parse_hex(b"9a826cff814626da47bc409383d83922f65ec3e890e3b41a60e89f3a864c2766"),
                y: gf2m::parse_hex(b"1e465ea7610428ec6b0b56be039dd73f3fe18d7d7731d60a18ff9224caaf43b76")
            }
        );
    }

    #[test]
    fn test_point_mul() {
        let mod257 = gf2m::compute_modulus(257, 12, 0, 0);
        let curve_a = gf2m::zero();
        let privd = gf2m::parse_hex(b"2A45EAFE4CD469F811737780C57253360FBCC58E134C9A1FDCD10B0E4529A143");

        let point = Point {
            x: gf2m::parse_hex(b"00AFF3EE09CB429284985849E20DE5742E194AA631490F62BA88702505629A6589"),
            y: gf2m::parse_hex(b"01B345BC134F27DA251EDFAE97B3F306B4E8B8CB9CF86D8651E4FB301EF8E1239C")
        };

        assert_eq!(
            curve::point_mul(&point, &privd, &mod257, &curve_a),
            Point {
                x: gf2m::parse_hex(b"8c3d388b1c51116cf0ed041718309b360f775d8df86e9fc141822e79a3b0da8b"),
                y: gf2m::parse_hex(b"a8624188d9f4ab0afafbde6230cd8cf7c28b38f42fcbb4021ff0c0244a5ddbbd")
            }
        );
    }

    #[test]
    fn test_point_expand() {
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

        let point_compressed_x = gf2m::from_bytes_le(&point_data);
        let point = curve::point_expand(&point_compressed_x, &curve);
        assert_eq!(point, Point {
            x: gf2m::parse_hex(b"44d59ab11eebf43534911d05992fc24fa8c6d0da68276eaf1bcde5dc6767705b527b007e700da85f1dc5f550392cabca624bbdf91bb7"),
            y: gf2m::parse_hex(b"6edb5b3e38bf271233378ac0fe3990289007928f56beb38a4f63843b9995afdd88a09c7da6935a4b43b0afde65a4ca9c159d72ed5275"),
        });
    }

    #[test]
    fn test_point_expand_even() {
        let curve = dstu_params::curve_431();
        let point_data: [u8; 54] = [
            0xb7, 0x1b, 0xf9, 0xbd, 0x4b, 0x62, 0xca, 0xab,
            0x2c, 0x39, 0x50, 0xf5, 0xc5, 0x1d, 0x5f, 0xa8,
            0x0d, 0x70, 0x7e, 0x00, 0x7b, 0x52, 0x5b, 0x70,
            0x67, 0x67, 0xdc, 0xe5, 0xcd, 0x1b, 0xaf, 0x6e,
            0x27, 0x68, 0xda, 0xd0, 0xc6, 0xa8, 0x4f, 0xc2,
            0x2f, 0x99, 0x05, 0x1d, 0x91, 0x34, 0x35, 0xf4,
            0xeb, 0x1e, 0xb1, 0x9a, 0xd5, 0x44
        ];

        let point_compressed_x = gf2m::from_bytes_le(&point_data);
        let point = curve::point_expand(&point_compressed_x, &curve);

        assert_eq!(point, Point {
            x: gf2m::parse_hex(b"44d59ab11eebf43534911d05992fc24fa8c6d0da68276eaf1bcde5dc6767705b527b007e700da85f1dc5f550392cabca624bbdf91bb7"),
            y: gf2m::parse_hex(b"2a0ec18f2654d32707a697c56716526738c142553e99dd2554ae61e7fef2df86dadb9c03d69ef2145e755a8e5c88615677d6cf1449c2"),
        });
    }
}
