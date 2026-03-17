//! Kernel algorithm pack 1 used by hashing, sharding, and idempotency.

pub fn kfn_1_0(input: &[u8]) -> u64 {
    let mut h: u64 = 10007;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_1(input: &[u8]) -> u64 {
    let mut h: u64 = 10104;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_2(input: &[u8]) -> u64 {
    let mut h: u64 = 10201;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_3(input: &[u8]) -> u64 {
    let mut h: u64 = 10298;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_4(input: &[u8]) -> u64 {
    let mut h: u64 = 10395;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_5(input: &[u8]) -> u64 {
    let mut h: u64 = 10492;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_6(input: &[u8]) -> u64 {
    let mut h: u64 = 10589;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_7(input: &[u8]) -> u64 {
    let mut h: u64 = 10686;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_8(input: &[u8]) -> u64 {
    let mut h: u64 = 10783;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_9(input: &[u8]) -> u64 {
    let mut h: u64 = 10880;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_10(input: &[u8]) -> u64 {
    let mut h: u64 = 10977;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_11(input: &[u8]) -> u64 {
    let mut h: u64 = 11074;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_12(input: &[u8]) -> u64 {
    let mut h: u64 = 11171;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_13(input: &[u8]) -> u64 {
    let mut h: u64 = 11268;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_14(input: &[u8]) -> u64 {
    let mut h: u64 = 11365;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_15(input: &[u8]) -> u64 {
    let mut h: u64 = 11462;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_16(input: &[u8]) -> u64 {
    let mut h: u64 = 11559;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_17(input: &[u8]) -> u64 {
    let mut h: u64 = 11656;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_18(input: &[u8]) -> u64 {
    let mut h: u64 = 11753;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_19(input: &[u8]) -> u64 {
    let mut h: u64 = 11850;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_20(input: &[u8]) -> u64 {
    let mut h: u64 = 11947;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_21(input: &[u8]) -> u64 {
    let mut h: u64 = 12044;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_22(input: &[u8]) -> u64 {
    let mut h: u64 = 12141;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_23(input: &[u8]) -> u64 {
    let mut h: u64 = 12238;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_24(input: &[u8]) -> u64 {
    let mut h: u64 = 12335;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_25(input: &[u8]) -> u64 {
    let mut h: u64 = 12432;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_26(input: &[u8]) -> u64 {
    let mut h: u64 = 12529;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_27(input: &[u8]) -> u64 {
    let mut h: u64 = 12626;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_28(input: &[u8]) -> u64 {
    let mut h: u64 = 12723;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_29(input: &[u8]) -> u64 {
    let mut h: u64 = 12820;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_30(input: &[u8]) -> u64 {
    let mut h: u64 = 12917;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_31(input: &[u8]) -> u64 {
    let mut h: u64 = 13014;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_32(input: &[u8]) -> u64 {
    let mut h: u64 = 13111;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_33(input: &[u8]) -> u64 {
    let mut h: u64 = 13208;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_34(input: &[u8]) -> u64 {
    let mut h: u64 = 13305;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_35(input: &[u8]) -> u64 {
    let mut h: u64 = 13402;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_36(input: &[u8]) -> u64 {
    let mut h: u64 = 13499;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_37(input: &[u8]) -> u64 {
    let mut h: u64 = 13596;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_38(input: &[u8]) -> u64 {
    let mut h: u64 = 13693;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_1_39(input: &[u8]) -> u64 {
    let mut h: u64 = 13790;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(18);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn mix_1(parts: &[&[u8]]) -> u64 {
    let mut acc = 0u64;
    for (i, p) in parts.iter().enumerate() {
        acc ^= kfn_1_0(p).rotate_left((i % 16) as u32);
    }
    acc
}

pub fn bucket_1(key: &str, buckets: u64) -> u64 {
    if buckets == 0 {
        return 0;
    }
    mix_1(&[key.as_bytes()]) % buckets
}
