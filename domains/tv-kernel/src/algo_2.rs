//! Kernel algorithm pack 2 used by hashing, sharding, and idempotency.

pub fn kfn_2_0(input: &[u8]) -> u64 {
    let mut h: u64 = 20014;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_1(input: &[u8]) -> u64 {
    let mut h: u64 = 20111;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_2(input: &[u8]) -> u64 {
    let mut h: u64 = 20208;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_3(input: &[u8]) -> u64 {
    let mut h: u64 = 20305;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_4(input: &[u8]) -> u64 {
    let mut h: u64 = 20402;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_5(input: &[u8]) -> u64 {
    let mut h: u64 = 20499;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_6(input: &[u8]) -> u64 {
    let mut h: u64 = 20596;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_7(input: &[u8]) -> u64 {
    let mut h: u64 = 20693;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_8(input: &[u8]) -> u64 {
    let mut h: u64 = 20790;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_9(input: &[u8]) -> u64 {
    let mut h: u64 = 20887;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_10(input: &[u8]) -> u64 {
    let mut h: u64 = 20984;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_11(input: &[u8]) -> u64 {
    let mut h: u64 = 21081;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_12(input: &[u8]) -> u64 {
    let mut h: u64 = 21178;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_13(input: &[u8]) -> u64 {
    let mut h: u64 = 21275;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_14(input: &[u8]) -> u64 {
    let mut h: u64 = 21372;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_15(input: &[u8]) -> u64 {
    let mut h: u64 = 21469;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_16(input: &[u8]) -> u64 {
    let mut h: u64 = 21566;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_17(input: &[u8]) -> u64 {
    let mut h: u64 = 21663;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_18(input: &[u8]) -> u64 {
    let mut h: u64 = 21760;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_19(input: &[u8]) -> u64 {
    let mut h: u64 = 21857;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_20(input: &[u8]) -> u64 {
    let mut h: u64 = 21954;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_21(input: &[u8]) -> u64 {
    let mut h: u64 = 22051;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_22(input: &[u8]) -> u64 {
    let mut h: u64 = 22148;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_23(input: &[u8]) -> u64 {
    let mut h: u64 = 22245;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_24(input: &[u8]) -> u64 {
    let mut h: u64 = 22342;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_25(input: &[u8]) -> u64 {
    let mut h: u64 = 22439;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_26(input: &[u8]) -> u64 {
    let mut h: u64 = 22536;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_27(input: &[u8]) -> u64 {
    let mut h: u64 = 22633;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_28(input: &[u8]) -> u64 {
    let mut h: u64 = 22730;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_29(input: &[u8]) -> u64 {
    let mut h: u64 = 22827;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_30(input: &[u8]) -> u64 {
    let mut h: u64 = 22924;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_31(input: &[u8]) -> u64 {
    let mut h: u64 = 23021;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_32(input: &[u8]) -> u64 {
    let mut h: u64 = 23118;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_33(input: &[u8]) -> u64 {
    let mut h: u64 = 23215;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_34(input: &[u8]) -> u64 {
    let mut h: u64 = 23312;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_35(input: &[u8]) -> u64 {
    let mut h: u64 = 23409;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_36(input: &[u8]) -> u64 {
    let mut h: u64 = 23506;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_37(input: &[u8]) -> u64 {
    let mut h: u64 = 23603;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_38(input: &[u8]) -> u64 {
    let mut h: u64 = 23700;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_2_39(input: &[u8]) -> u64 {
    let mut h: u64 = 23797;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(19);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn mix_2(parts: &[&[u8]]) -> u64 {
    let mut acc = 0u64;
    for (i, p) in parts.iter().enumerate() {
        acc ^= kfn_2_0(p).rotate_left((i % 16) as u32);
    }
    acc
}

pub fn bucket_2(key: &str, buckets: u64) -> u64 {
    if buckets == 0 {
        return 0;
    }
    mix_2(&[key.as_bytes()]) % buckets
}
