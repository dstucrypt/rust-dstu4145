const FIELD_SIZE: usize = 16;
const FIELD_BYTES: usize = 64;
const WORD_SIZE: usize = 32;

pub type Field = [u32; FIELD_SIZE];
pub type FieldMul = [u32; FIELD_SIZE * 2];
pub type FieldBytes = [u8; FIELD_BYTES];

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

pub fn compute_modulus(p1: usize, k1: usize, k2: usize, k3: usize) -> Field {
    let mut modulus: Field = [0; FIELD_SIZE];
    set_bit(&mut modulus, 0);
    set_bit(&mut modulus, p1);
    set_bit(&mut modulus, k1);
    set_bit(&mut modulus, k2);
    set_bit(&mut modulus, k3);
    return modulus;
}

#[inline]
fn max(a: usize, b: usize)-> usize {
    return if a > b {
        a
    } else {
        b
    };
}

static BIT_SIZE: [usize; 256] = [
      0, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5,
      5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
      6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7,
      7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
      7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
      7, 7, 7, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
      8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
      8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
      8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
      8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
      8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8];

#[inline]
fn word_bits(value: u32) -> usize {
    if (value & 0xffff0000) != 0 {
      if (value & 0xff000000) != 0 {
        return BIT_SIZE[(value >> 24) as usize] + 24;
      } else {
        return BIT_SIZE[(value >> 16) as usize] + 16;
      }
    } else {
      if (value & 0xff00) != 0 {
        return BIT_SIZE[(value >> 8) as usize] + 8;
      } else {
        return BIT_SIZE[value as usize];
      }
    }
}

#[inline]
pub fn bit_size(value: &[u32]) -> usize {
    let mut max_word = 0;

    for size in 0..value.len() {
        let word = value[size];
        let mask;
        if word == 0 {
            mask = 0;
        }
        else {
            mask = 0xFF_FF;
        }

        max_word = max(max_word, mask & size);
    }

    let word = value[max_word];
    let max_size = max_word << 5;
    return max_size + word_bits(word);
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

pub fn reduce(value: &[u32], modulus: &Field) -> Field {
    let mut ret: FieldMul = [0; FIELD_SIZE * 2];
    for i in 0..value.len() {
        ret[i] = value[i];
    }
    let mut ret_field: Field = [0; FIELD_SIZE];
    let mut bigmodulus: FieldMul = [0; FIELD_SIZE * 2];
    for i in 0..FIELD_SIZE {
        bigmodulus[i] = modulus[i];
    }

    let mut size_difference = (bit_size(&ret) as i32) - (bit_size(modulus) as i32);
    while size_difference >= 0 {
        let mask = shll(&bigmodulus, size_difference as usize);
        ret = addll(&ret, &mask);
        size_difference = (bit_size(&ret) as i32) - (bit_size(modulus) as i32);
    }

    for i in 0..FIELD_SIZE {
        ret_field[i] = ret[i];
    }

    return ret_field;
}

#[inline]
fn mul_1x1_u32 (a: u32, b: u32)-> (u32, u32) {
    let top2b = a >> 30;

    let a1 = a & (0x3FFFFFFF);
    let a2 = a1 << 1;
    let a4 = a2 << 1;

    let tab  =  [
        0, a1, a2, a1^a2,
        a4, a1^a4, a2^a4, a1^a2^a4
    ];

    let mut s;
    let mut l;
    let mut h;
    s = tab[(b       & 0x7) as usize]; l  = s;
    s = tab[(b >>  3 & 0x7) as usize]; l ^= s <<  3; h  = s >> 29;
    s = tab[(b >>  6 & 0x7) as usize]; l ^= s <<  6; h ^= s >> 26;
    s = tab[(b >>  9 & 0x7) as usize]; l ^= s <<  9; h ^= s >> 23;
    s = tab[(b >> 12 & 0x7) as usize]; l ^= s << 12; h ^= s >> 20;
    s = tab[(b >> 15 & 0x7) as usize]; l ^= s << 15; h ^= s >> 17;
    s = tab[(b >> 18 & 0x7) as usize]; l ^= s << 18; h ^= s >> 14;
    s = tab[(b >> 21 & 0x7) as usize]; l ^= s << 21; h ^= s >> 11;
    s = tab[(b >> 24 & 0x7) as usize]; l ^= s << 24; h ^= s >>  8;
    s = tab[(b >> 27 & 0x7) as usize]; l ^= s << 27; h ^= s >>  5;
    s = tab[(b >> 30) as usize];       l ^= s << 30; h ^= s >>  2;

    let b30 = b << 30;
    let b31 = b << 31;
    let b2 = b >> 2;
    let b1 = b >> 1;

    let top = [
        0, b30, b31, b30 ^ b31,
        0, b2, b1, b2 ^ b1,
    ];

    l ^= top[top2b as usize];
    h ^= top[(top2b | 0x4) as usize];

    return (l, h);
}

#[inline]
fn mul_2x2_u32(a1: u32, a0: u32, b1: u32, b0: u32) -> (u32, u32, u32, u32) {

    let (z0l, z0h) = mul_1x1_u32(a0, b0);
    let (z1l, z1h) = mul_1x1_u32(a1, b1);
    let (z2l, z2h) = mul_1x1_u32(a0 ^ a1, b0 ^ b1);

    let z1l = z1l ^ z0h ^ z1h ^ z2h;
    let z0h = z1h ^ z1l ^ z0l  ^ z2l ^ z2h;
    return (z0l, z0h, z1l, z1h);
}

pub fn mul_1x1(value_a: &Field, value_b: &Field) -> FieldMul {
    let mut result: FieldMul = [0; FIELD_SIZE * 2];

    for j in 0..FIELD_SIZE {
        for i in 0..FIELD_SIZE {
            let (l, h) = mul_1x1_u32(value_a[j], value_b[i]);
            result[j+i] ^= l;
            result[j+i + 1] ^= h;
        }
    }

    return result;
}

pub fn mul(value_a: &Field, value_b: &Field) -> FieldMul {
    let mut result: FieldMul = [0; FIELD_SIZE * 2];

    let mut j = 0;
    let mut i;

    while j < (FIELD_SIZE - 2) {
        i = 0;
        let y0 = value_b[j];
        let y1 = value_b[j + 1];

        while i < FIELD_SIZE {
            let x0 = value_a[i];
            let x1 = value_a[i + 1];

            let (l0, h0, l1, h1) = mul_2x2_u32(x1, x0, y1, y0);
            result[j+i] ^= l0;
            result[j+i + 1] ^= h0;
            result[j+i + 2] ^= l1;
            result[j+i + 3] ^= h1;

            i+=2;
        }

        j += 2
    }

    return result;
}

pub fn mul_testbit(value_a: &Field, value_b: &Field) -> FieldMul {
    let mut result: FieldMul = [0; FIELD_SIZE * 2];
    let mut one: Field = [0; FIELD_SIZE];
    one[FIELD_SIZE - 1] = 1;

    let mut temp_b = addl(&result, value_b);
    for j in 0..bit_size(value_a) {
        if has_bit(value_a, j) {
            result = addll(&result, &temp_b);
        }
        temp_b = shll(&temp_b, 1);
    }

    return result;
}

pub fn sqr(value: &Field) -> FieldMul {
    return mul(value, value);
}

pub fn neg(value: &Field, modulus: &Field) -> Field {
    let mut b: Field = [0; FIELD_SIZE]; b[0] = 1;
    let mut c: Field = [0; FIELD_SIZE];
    let mut u = reduce(value, modulus);
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
        u = add(&u, &shl(&v, j as usize));
        b = add(&b, &shl(&c, j as usize));

        i = i + 1;
    }
    return b;
}

pub fn add(value_a: &Field, value_b: &Field) -> Field {
    let mut ret : Field = [0; FIELD_SIZE];
    for i in 0..FIELD_SIZE {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn addl(value_a: &FieldMul, value_b: &Field) -> FieldMul {
    let mut ret = value_a.clone();
    for i in 0..FIELD_SIZE {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn addll(value_a: &FieldMul, value_b: &FieldMul) -> FieldMul {
    let mut ret = value_a.clone();
    for i in 0..(FIELD_SIZE * 2) {
        ret[i] = value_a[i] ^ value_b[i];
    }
    return ret;
}

pub fn truncate(value: &Field, size: usize) -> Field {
    let mut result = value.clone();
    while size <= bit_size(&result) {
        let mut mask = zero();
        set_bit(&mut mask, bit_size(&result) - 1);
        result = add(&result, &mask);
    }
    return result;
}

pub fn trace(value: &Field, modulus: &Field) -> u32 {
    let mut result = value.clone();
    let mut i = 1;
    let bits = bit_size(modulus) - 1;
    while i < bits {
        result = reduce(&mul(&result, &result), modulus);
        result = add(&result, value);
        i = i + 1;
    }
    return result[0];
}

pub fn squad_odd(value: &Field, modulus: &Field, field_m: usize) -> Field {
    let val_a = reduce(value, modulus);
    let mut val_z = val_a.clone();
    let half_m = (field_m - 1) / 2;
    let mut i = 1;

    while i <= half_m {
        val_z = reduce(&mul(&val_z, &val_z), modulus);
        val_z = reduce(&mul(&val_z, &val_z), modulus);
        val_z = add(&val_z, &val_a);
        i = i + 1;
    }

    let val_w = add(&reduce(&mul(&val_z, &val_z), modulus), &val_z);

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

pub fn zero_one(value: &Field) -> bool {
    return value[0] == 1;
}
