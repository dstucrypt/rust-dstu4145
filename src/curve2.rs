use gf2m;
use gf2m::Field;

#[derive(Clone, Debug, Hash)]
pub struct Point {
    pub x: Field,
    pub y: Field,
}

impl PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        return self.x.eq(&other.x) && self.y.eq(&other.y);
    }
}

pub struct Curve {
    pub param_a: Field,
    pub param_b: Field,
    pub order: Field,
    pub base: Point,
    pub field_m: usize,
    pub field_k1: usize,
    pub field_k2: usize,
    pub field_k3: usize,
    pub modulus: Field,
}

pub fn infinity()-> Point {
    return Point {x: gf2m::zero(), y: gf2m::zero()};
}

pub fn at_infinity(value_x: &Field, value_y: &Field) -> bool {
    return gf2m::is_zero(value_x) && gf2m::is_zero(value_y);
}

pub fn point_add(point_a: &Point, point_b: &Point,
                 modulus: &Field,
                 curve_a: &Field) -> Point {

    if at_infinity(&point_a.x, &point_a.y) {
        return point_b.clone();
    }

    if at_infinity(&point_b.x, &point_b.y) {
        return point_a.clone();
    }

    /* Ref: https://hyperelliptic.org/EFD/g12o/auto-shortw-affine.html
       lambda = (Y1+Y2)/(X1+X2)
       X3 = lambda2+lambda+X1+X2+a2
       Y3 = lambda*(X1+X3)+X3+Y1 */

    let lbd;
    let value_cx;

    if point_a.x.eq(&point_b.x) == false {
        let neg_abx = gf2m::neg_bytes(&gf2m::add_bytes(&point_a.x, &point_b.x), modulus);
        lbd = gf2m::reduce_bytes(
            &gf2m::mul_bytes(
                &gf2m::add_bytes(&point_a.y, &point_b.y),
                &neg_abx,
            ),
            modulus
        );
        let temp_cx = gf2m::add_bytes(
            curve_a,
            &gf2m::reduce_bytes(&gf2m::mul_bytes(&lbd, &lbd), modulus)
        );
        let temp_cx = gf2m::add_bytes(&temp_cx, &lbd);
        let temp_cx = gf2m::add_bytes(&temp_cx, &point_a.x);
        value_cx = gf2m::add_bytes(&temp_cx, &point_b.x);
    }
    else if point_a.y.eq(&point_b.y) == false {
        return infinity();
    }
    else if gf2m::is_zero(&point_a.x) {
        return infinity();
    }
    else {
        let neg_ax = gf2m::neg_bytes(&point_a.x, modulus);
        lbd = gf2m::add_bytes(
            &point_a.x,
            &gf2m::reduce_bytes(
                &gf2m::mul_bytes(&point_a.y, &neg_ax),
                modulus
            )
        );
        let temp = gf2m::add_bytes(
            curve_a,
            &gf2m::reduce_bytes(
                &gf2m::mul_bytes(&lbd, &lbd),
                modulus
            )
        );
        value_cx = gf2m::add_bytes(&temp, &lbd);;
    }

    let value_cy = gf2m::reduce_bytes(
        &gf2m::mul_bytes(&gf2m::add_bytes(&point_b.x, &value_cx), &lbd),
        modulus
    );
    let value_cy = gf2m::add_bytes(&value_cy, &value_cx);
    let value_cy = gf2m::add_bytes(&value_cy, &point_b.y);

    return Point {x: value_cx, y: value_cy};
}

pub fn point_dbl(point_a: &Point,
                 modulus: &Field,
                 curve_a: &Field) -> Point {

    if at_infinity(&point_a.x, &point_a.y) {
        return point_a.clone();
    }

    /* Ref: https://hyperelliptic.org/EFD/g12o/auto-shortw-affine.html
       lambda = X1+Y1/X1
       X3 = lambda2+lambda+a2
       Y3 = lambda*(X1+X3)+X3+Y1 */

    let lbd;
    let value_cx;

    let neg_ax = gf2m::neg_bytes(&point_a.x, modulus);
    lbd = gf2m::add_bytes(
        &point_a.x,
        &gf2m::reduce_bytes(
            &gf2m::mul_bytes(&point_a.y, &neg_ax),
            modulus
        )
    );
    let temp = gf2m::add_bytes(
        curve_a,
        &gf2m::reduce_bytes(
            &gf2m::sqr_bytes(&lbd),
            modulus
        )
    );
    value_cx = gf2m::add_bytes(&temp, &lbd);;

    let value_cy = gf2m::reduce_bytes(
        &gf2m::mul_bytes(&gf2m::add_bytes(&point_a.x, &value_cx), &lbd),
        modulus
    );
    let value_cy = gf2m::add_bytes(&value_cy, &value_cx);
    let value_cy = gf2m::add_bytes(&value_cy, &point_a.y);

    return Point {x: value_cx, y: value_cy};
}

// FIXME: negative mul impossible
pub fn point_mul(point: &Point, factor: &Field,
                 modulus: &Field,
                 curve_a: &Field) -> Point {

    if gf2m::is_zero(factor) {
        return infinity();
    }

    let mut j = gf2m::bit_size(factor) as i32;

    let mut point_r0 = infinity();
    let mut point_r1 = point.clone();

    while j >= 0 {
        if gf2m::has_bit(factor, j as usize) {
             point_r0 = point_add(
                &point_r0, &point_r1,
                modulus, curve_a
            );
            point_r1 = point_dbl(&point_r1, modulus, curve_a);
        }
        else {

            point_r1 = point_add(
                &point_r0, &point_r1,
                modulus, curve_a
            );
            point_r0 = point_dbl(&point_r0, modulus, curve_a);
        }
        j = j - 1;
    }

    return point_r0;
}

pub fn point_expand(compressed: &Field, curve: &Curve)-> Point {

    let mut value = compressed.clone();
    if gf2m::is_zero(compressed) {
        let mulpb = gf2m::reduce_bytes(
            &gf2m::mul_bytes(&curve.param_b, &curve.param_b),
            &curve.modulus
        );
        return Point {x: value, y: mulpb}
    }
    let k = value[0] & 1;

    value[0] = value[0] & 0xFF_FF_FF_FE;

    let trace = gf2m::trace_bytes(&value, &curve.modulus);
    if (trace == 1 && gf2m::is_zero(&curve.param_a)) ||
       (trace == 0 && !gf2m::is_zero(&curve.param_a)) {
        value[0] = value[0] | 1;
    }
    let x2 = gf2m::reduce_bytes(&gf2m::mul_bytes(&value, &value), &curve.modulus);
    let mut y = gf2m::reduce_bytes(&gf2m::mul_bytes(&x2, &value), &curve.modulus);

    if !gf2m::is_zero(&curve.param_a) {
        y = gf2m::add_bytes(&y, &x2);
    }

    y = gf2m::add_bytes(&y, &curve.param_b);
    let invx2 = gf2m::neg_bytes(&x2, &curve.modulus);
    y = gf2m::reduce_bytes(&gf2m::mul_bytes(&y, &invx2), &curve.modulus);

    y = gf2m::reduce_bytes(&
        gf2m::squad_odd_bytes(&y, &curve.modulus, curve.field_m),
        &curve.modulus
    );

    let trace_y = gf2m::trace_bytes(&y, &curve.modulus);

    if (k == 1 && trace_y == 0) ||
       (k == 0 && trace_y == 1) {
        y[0] = y[0] ^ 1;
    }

    y = gf2m::reduce_bytes(&gf2m::mul_bytes(&y, &value), &curve.modulus);

    return Point {x: value, y: y};
}
