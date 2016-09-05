extern crate num;

use num::{BigUint, One, Zero, ToPrimitive};
use std::ops::{BitOr, BitAnd, BitXor, Shl};
use std::cmp::Ordering::{Less};

const FIELD_SIZE: usize = 16;
const FIELD_BYTES: usize = 64;
const WORD_SIZE: usize = 32;

pub type Field = [u32; FIELD_SIZE];
pub type FieldMul = [u32; FIELD_SIZE * 2];
pub type FieldBytes = [u8; FIELD_BYTES];

pub fn compute_modulus(p1: usize, k1: usize, k2: usize, k3: usize) -> BigUint {
    let one =  BigUint::one();
    let modulus = BigUint::one().bitor(one.shl(p1));
    let modulus = modulus.bitor(BigUint::one().shl(k1));
    let modulus = modulus.bitor(BigUint::one().shl(k2));
    let modulus = modulus.bitor(BigUint::one().shl(k3));

    return modulus;
}


pub fn to_bytes_le(words: &Field) -> [u8; FIELD_BYTES] {
    let mut ret = [0; FIELD_BYTES];
    for x in 0..FIELD_SIZE {
        ret[x * 4] = (words[x] & 0xFF) as u8;
        ret[(x * 4) + 1] = ((words[x] >> 8) & 0xFF) as u8;
        ret[(x * 4) + 2] = ((words[x] >> 16) & 0xFF) as u8;
        ret[(x * 4) + 3] = ((words[x] >> 24) & 0xFF) as u8;
    }
    return ret;
}

pub fn from_bytes_le(data: &[u8]) -> Field {
    let mut ret: Field = [0; FIELD_SIZE];
    for x in 0..data.len() {
        ret[x >> 2] |= (data[x] as u32) << ((x % 4) * 8);
    }
    return ret;
}

#[inline]
fn set_bit(words: &mut Field, bit: usize) {
    let word = bit / WORD_SIZE;
    let wbit = bit % 32;
    words[word] |= 1 << wbit;
}

#[inline]
pub fn has_bit(words: &[u32], bit: usize)-> bool {
    let word = bit / WORD_SIZE;
    let wbit = bit % 32;
    return (words[word] & 1 << wbit) != 0;
}

pub fn compute_modulus_bytes(p1: usize, k1: usize, k2: usize, k3: usize) -> Field {
    let mut modulus: Field = [0; FIELD_SIZE];
    set_bit(&mut modulus, 0);
    set_bit(&mut modulus, p1);
    set_bit(&mut modulus, k1);
    set_bit(&mut modulus, k2);
    set_bit(&mut modulus, k3);
    return modulus;
}

pub fn fmod(value: BigUint, modulus: &BigUint) -> BigUint {
	let mut value = match value.cmp(modulus) {
		Less => return value,
        _ => value,
	};
    while value.bits() >= modulus.bits() {
        let mask = modulus.shl(value.bits() - modulus.bits());
        value = value.bitxor(&mask);
    }
    return value;
}

// duh
pub fn bit_size(value: &Field) -> usize {
    let mut size = (FIELD_SIZE * 32) - 1;

    while size > 0 {
        if has_bit(value, size) {
            return size + 1;
        }
        size -= 1;
    }
    return size;
}

fn bitl_size(value: &FieldMul) -> usize {
    let mut size = (FIELD_SIZE * 2 * 32) - 1;

    while size > 0 {
        if has_bit(value, size) {
            return size + 1;
        }
        size -= 1;
    }
    return size;
}

pub fn shl(value: &Field, shift: usize) -> Field {
    let word_shift = shift / WORD_SIZE;
    let bit_shift = shift % WORD_SIZE;

    let rbit_mask: u32 = if bit_shift > 0 {
        0xFF_FF_FF_FF
    } else {
        0x0
    };

    let rbit_shift: usize = if bit_shift > 0 {
        bit_shift
    } else {
        WORD_SIZE
    };
    let rbit_shift = WORD_SIZE - rbit_shift;

    let mut ret: Field = [0; FIELD_SIZE];

    let last = FIELD_SIZE - word_shift;

    ret[word_shift] = value[0] << bit_shift;
    for i in 1..last {
        ret[i + word_shift] = (value[i] << bit_shift) |
                 rbit_mask & (value[i - 1] >> rbit_shift);
    }
    return ret;
}

pub fn shll(value: &FieldMul, shift: usize) -> FieldMul {
    let word_shift = shift / WORD_SIZE;
    let bit_shift = shift % WORD_SIZE;

    let rbit_mask: u32 = if bit_shift > 0 {
        0xFF_FF_FF_FF
    } else {
        0x0
    };

    let rbit_shift: usize = if bit_shift > 0 {
        bit_shift
    } else {
        WORD_SIZE
    };
    let rbit_shift = WORD_SIZE - rbit_shift;

    let mut ret: FieldMul = [0; FIELD_SIZE * 2];

    let last = (FIELD_SIZE * 2) - word_shift;

    ret[word_shift] = value[0] << bit_shift;
    for i in 1..last {
        ret[i + word_shift] = (value[i] << bit_shift) |
                 rbit_mask & (value[i - 1] >> rbit_shift);
    }
    return ret;
}

pub fn reduce_bytes(value: &[u32], modulus: &Field) -> Field {
    let mut ret: FieldMul = [0; FIELD_SIZE * 2];
    for i in 0..value.len() {
        ret[i] = value[i];
    }
    let mut ret_field: Field = [0; FIELD_SIZE];
    let mut bigmodulus: FieldMul = [0; FIELD_SIZE * 2];
    for i in 0..FIELD_SIZE {
        bigmodulus[i] = modulus[i];
    }

    while bitl_size(&ret) >= bit_size(modulus) {
        let mask = shll(&bigmodulus, bitl_size(&ret) - bit_size(modulus));
        ret = addll_bytes(&ret, &mask);
    }

    for i in 0..FIELD_SIZE {
        ret_field[i] = ret[i];
    }

    return ret_field;
}

pub fn mul(value_a: &BigUint, value_b: &BigUint) -> BigUint {
    let mut result = BigUint::zero();
    let mut j = 0;
    let mut temp_b = value_b.clone();
    while j < value_a.bits() {
        let mask = BigUint::one().shl(j);
        let test = value_a.bitand(mask);
        if test.is_zero() == false {
            result = result.bitxor(&temp_b);
        }
        temp_b = temp_b.shl(1);
        j = j + 1;
    }
    return result;
}

pub fn mul_bytes(value_a: &Field, value_b: &Field) -> FieldMul {
    let mut result: FieldMul = [0; FIELD_SIZE * 2];
    let mut one: Field = [0; FIELD_SIZE];
    one[FIELD_SIZE - 1] = 1;

    let mut temp_b = addl_bytes(&result, value_b);
    for j in 0..bit_size(value_a) {
        if has_bit(value_a, j) {
            result = addll_bytes(&result, &temp_b);
        }
        temp_b = shll(&temp_b, 1);
    }

    return result;
}

pub fn sqr(value: &BigUint) -> BigUint {
    return mul(value, value);
}

pub fn sqr_bytes(value: &Field) -> FieldMul {
    return mul_bytes(value, value);
}

pub fn neg(value: BigUint, modulus: &BigUint) -> BigUint {
    let mut b = BigUint::one();
    let mut c = BigUint::zero();
    let mut u = fmod(value, modulus);
    let mut v = modulus.clone();

    while u.bits() > 1 {
        let mut j: i32 = (u.bits() as i32) - (v.bits() as i32);

        if j < 0 {
            let temp = u;
            u = v;
            v = temp;

            let temp = c;
            c = b;
            b = temp;

            j = -j;
        }
        let ref vref = v;
        let ref cref = c;
        u = add(&u, &vref.shl(j as usize));
        b = add(&b, &cref.shl(j as usize));
    }

    return b;
}

pub fn neg_bytes(value: &Field, modulus: &Field) -> Field {
    let mut b: Field = [0; FIELD_SIZE]; b[0] = 1;
    let mut c: Field = [0; FIELD_SIZE];
    let mut u = reduce_bytes(value, modulus);
    let mut v = modulus.clone();

    let mut i = 0;
    while bit_size(&u) > 1 {
        let mut j: i32 = (bit_size(&u) as i32) - (bit_size(&v) as i32);

        if j < 0 {
            let temp = u;
            u = v;
            v = temp;

            let temp = c;
            c = b;
            b = temp;


            j = -j;
        }
        u = add_bytes(&u, &shl(&v, j as usize));
        b = add_bytes(&b, &shl(&c, j as usize));

        i = i + 1;
    }
    return b;
}


pub fn add(value_a: &BigUint, value_b: &BigUint)-> BigUint {
    return value_a.bitxor(value_b);
}

pub fn add_bytes(value_a: &Field, value_b: &Field) -> Field {
    let mut ret : Field = [0; FIELD_SIZE];
    for i in 0..FIELD_SIZE {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn addl_bytes(value_a: &FieldMul, value_b: &Field) -> FieldMul {
    let mut ret = value_a.clone();
    for i in 0..FIELD_SIZE {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn addll_bytes(value_a: &FieldMul, value_b: &FieldMul) -> FieldMul {
    let mut ret = value_a.clone();
    for i in 0..(FIELD_SIZE * 2) {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn truncate(value: &BigUint, size: usize) -> BigUint {
    let one = &BigUint::one();
    let mut result = value.clone();
    while size <= result.bits() {
        result = add(&result, &one.shl(result.bits() - 1));
    }
    return result;
}

pub fn trace(value: &BigUint, modulus: &BigUint) -> u64 {
    let mut result = value.clone();
    let mut i = 1;
    let bits = modulus.bits() - 1;
    while i < bits {
        result = fmod(mul(&result, &result), modulus);
        result = add(&result, value);
        i = i + 1;
    }
    return result.to_u64().unwrap();
}

pub fn trace_bytes(value: &Field, modulus: &Field) -> u32 {
    let mut result = value.clone();
    let mut i = 1;
    let bits = bit_size(modulus) - 1;
    while i < bits {
        result = reduce_bytes(&mul_bytes(&result, &result), modulus);
        result = add_bytes(&result, value);
        i = i + 1;
    }
    return result[0];
}

pub fn squad_odd(value: &BigUint, modulus: &BigUint, field_m: usize) -> BigUint {
    let val_a = fmod(value.clone(), modulus);
    let mut val_z = val_a.clone();
    let half_m = (field_m - 1) / 2;
    let mut i = 1;

    while i <= half_m {
        val_z = fmod(mul(&val_z, &val_z), modulus);
        val_z = fmod(mul(&val_z, &val_z), modulus);
        val_z = add(&val_z, &val_a);
        i = i + 1;
    }

    let val_w = add(&fmod(mul(&val_z, &val_z), modulus), &val_z);

    assert_eq!(val_w, val_a);

    return val_z;
}

pub fn squad_odd_bytes(value: &Field, modulus: &Field, field_m: usize) -> Field {
    let val_a = reduce_bytes(value, modulus);
    let mut val_z = val_a.clone();
    let half_m = (field_m - 1) / 2;
    let mut i = 1;

    while i <= half_m {
        val_z = reduce_bytes(&mul_bytes(&val_z, &val_z), modulus);
        val_z = reduce_bytes(&mul_bytes(&val_z, &val_z), modulus);
        val_z = add_bytes(&val_z, &val_a);
        i = i + 1;
    }

    let val_w = add_bytes(&reduce_bytes(&mul_bytes(&val_z, &val_z), modulus), &val_z);

    assert_eq!(val_w, val_a);

    return val_z;
}

pub fn parse_hex(data: &[u8])-> Field {
    let mut ret : Field = [0; FIELD_SIZE];
    for i in 0..data.len() {
        let n8 = match data[i] as char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 0xA,
            'a' => 0xA,
            'B' => 0xB,
            'b' => 0xB,
            'C' => 0xC,
            'c' => 0xC,
            'D' => 0xD,
            'd' => 0xD,
            'E' => 0xE,
            'e' => 0xe,
            'F' => 0xF,
            'f' => 0xf,
            _ => 0,
        };
        let pos = data.len() - i - 1;
        let posword = pos / 8;
        let posbyte = pos % 8;

        ret[posword] = ret[posword] | (n8 << (posbyte * 4));
    }
    return ret;
}

pub fn zero() -> Field {
    return [0; FIELD_SIZE];
}

pub fn one() -> Field {
    let mut ret = [0; FIELD_SIZE];
    ret[0] = 1;
    return ret;
}

pub fn is_zero(value: &Field) -> bool {
    let mut ret = true;
    for i in 0..FIELD_SIZE {
        ret = value[i] == 0 && ret;
    }
    return ret;
}
