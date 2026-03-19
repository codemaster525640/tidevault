//! Kernel algorithm pack 6 used by hashing, sharding, and idempotency.

pub fn kfn_6_0(input: &[u8]) -> u64 {
    let mut h: u64 = 60042;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_1(input: &[u8]) -> u64 {
    let mut h: u64 = 60139;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_2(input: &[u8]) -> u64 {
    let mut h: u64 = 60236;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_3(input: &[u8]) -> u64 {
    let mut h: u64 = 60333;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_4(input: &[u8]) -> u64 {
    let mut h: u64 = 60430;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_5(input: &[u8]) -> u64 {
    let mut h: u64 = 60527;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_6(input: &[u8]) -> u64 {
    let mut h: u64 = 60624;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_7(input: &[u8]) -> u64 {
    let mut h: u64 = 60721;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_8(input: &[u8]) -> u64 {
    let mut h: u64 = 60818;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_9(input: &[u8]) -> u64 {
    let mut h: u64 = 60915;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_10(input: &[u8]) -> u64 {
    let mut h: u64 = 61012;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_11(input: &[u8]) -> u64 {
    let mut h: u64 = 61109;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_12(input: &[u8]) -> u64 {
    let mut h: u64 = 61206;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_13(input: &[u8]) -> u64 {
    let mut h: u64 = 61303;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_14(input: &[u8]) -> u64 {
    let mut h: u64 = 61400;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_15(input: &[u8]) -> u64 {
    let mut h: u64 = 61497;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_16(input: &[u8]) -> u64 {
    let mut h: u64 = 61594;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_17(input: &[u8]) -> u64 {
    let mut h: u64 = 61691;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_18(input: &[u8]) -> u64 {
    let mut h: u64 = 61788;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_19(input: &[u8]) -> u64 {
    let mut h: u64 = 61885;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_20(input: &[u8]) -> u64 {
    let mut h: u64 = 61982;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_21(input: &[u8]) -> u64 {
    let mut h: u64 = 62079;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_22(input: &[u8]) -> u64 {
    let mut h: u64 = 62176;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_23(input: &[u8]) -> u64 {
    let mut h: u64 = 62273;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_24(input: &[u8]) -> u64 {
    let mut h: u64 = 62370;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_25(input: &[u8]) -> u64 {
    let mut h: u64 = 62467;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_26(input: &[u8]) -> u64 {
    let mut h: u64 = 62564;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_27(input: &[u8]) -> u64 {
    let mut h: u64 = 62661;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_28(input: &[u8]) -> u64 {
    let mut h: u64 = 62758;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_29(input: &[u8]) -> u64 {
    let mut h: u64 = 62855;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_30(input: &[u8]) -> u64 {
    let mut h: u64 = 62952;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_31(input: &[u8]) -> u64 {
    let mut h: u64 = 63049;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_32(input: &[u8]) -> u64 {
    let mut h: u64 = 63146;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_33(input: &[u8]) -> u64 {
    let mut h: u64 = 63243;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_34(input: &[u8]) -> u64 {
    let mut h: u64 = 63340;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_35(input: &[u8]) -> u64 {
    let mut h: u64 = 63437;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_36(input: &[u8]) -> u64 {
    let mut h: u64 = 63534;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_37(input: &[u8]) -> u64 {
    let mut h: u64 = 63631;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_38(input: &[u8]) -> u64 {
    let mut h: u64 = 63728;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_6_39(input: &[u8]) -> u64 {
    let mut h: u64 = 63825;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(23);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn mix_6(parts: &[&[u8]]) -> u64 {
    let mut acc = 0u64;
    for (i, p) in parts.iter().enumerate() {
        acc ^= kfn_6_0(p).rotate_left((i % 16) as u32);
    }
    acc
}

pub fn bucket_6(key: &str, buckets: u64) -> u64 {
    if buckets == 0 {
        return 0;
    }
    mix_6(&[key.as_bytes()]) % buckets
}
