//! Kernel algorithm pack 5 used by hashing, sharding, and idempotency.

pub fn kfn_5_0(input: &[u8]) -> u64 {
    let mut h: u64 = 50035;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_1(input: &[u8]) -> u64 {
    let mut h: u64 = 50132;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_2(input: &[u8]) -> u64 {
    let mut h: u64 = 50229;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_3(input: &[u8]) -> u64 {
    let mut h: u64 = 50326;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_4(input: &[u8]) -> u64 {
    let mut h: u64 = 50423;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_5(input: &[u8]) -> u64 {
    let mut h: u64 = 50520;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_6(input: &[u8]) -> u64 {
    let mut h: u64 = 50617;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_7(input: &[u8]) -> u64 {
    let mut h: u64 = 50714;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_8(input: &[u8]) -> u64 {
    let mut h: u64 = 50811;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_9(input: &[u8]) -> u64 {
    let mut h: u64 = 50908;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_10(input: &[u8]) -> u64 {
    let mut h: u64 = 51005;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_11(input: &[u8]) -> u64 {
    let mut h: u64 = 51102;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_12(input: &[u8]) -> u64 {
    let mut h: u64 = 51199;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_13(input: &[u8]) -> u64 {
    let mut h: u64 = 51296;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_14(input: &[u8]) -> u64 {
    let mut h: u64 = 51393;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_15(input: &[u8]) -> u64 {
    let mut h: u64 = 51490;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_16(input: &[u8]) -> u64 {
    let mut h: u64 = 51587;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_17(input: &[u8]) -> u64 {
    let mut h: u64 = 51684;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_18(input: &[u8]) -> u64 {
    let mut h: u64 = 51781;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_19(input: &[u8]) -> u64 {
    let mut h: u64 = 51878;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_20(input: &[u8]) -> u64 {
    let mut h: u64 = 51975;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_21(input: &[u8]) -> u64 {
    let mut h: u64 = 52072;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_22(input: &[u8]) -> u64 {
    let mut h: u64 = 52169;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_23(input: &[u8]) -> u64 {
    let mut h: u64 = 52266;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_24(input: &[u8]) -> u64 {
    let mut h: u64 = 52363;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_25(input: &[u8]) -> u64 {
    let mut h: u64 = 52460;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_26(input: &[u8]) -> u64 {
    let mut h: u64 = 52557;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_27(input: &[u8]) -> u64 {
    let mut h: u64 = 52654;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_28(input: &[u8]) -> u64 {
    let mut h: u64 = 52751;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_29(input: &[u8]) -> u64 {
    let mut h: u64 = 52848;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_30(input: &[u8]) -> u64 {
    let mut h: u64 = 52945;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_31(input: &[u8]) -> u64 {
    let mut h: u64 = 53042;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_32(input: &[u8]) -> u64 {
    let mut h: u64 = 53139;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_33(input: &[u8]) -> u64 {
    let mut h: u64 = 53236;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_34(input: &[u8]) -> u64 {
    let mut h: u64 = 53333;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_35(input: &[u8]) -> u64 {
    let mut h: u64 = 53430;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_36(input: &[u8]) -> u64 {
    let mut h: u64 = 53527;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_37(input: &[u8]) -> u64 {
    let mut h: u64 = 53624;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_38(input: &[u8]) -> u64 {
    let mut h: u64 = 53721;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_5_39(input: &[u8]) -> u64 {
    let mut h: u64 = 53818;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(22);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn mix_5(parts: &[&[u8]]) -> u64 {
    let mut acc = 0u64;
    for (i, p) in parts.iter().enumerate() {
        acc ^= kfn_5_0(p).rotate_left((i % 16) as u32);
    }
    acc
}

pub fn bucket_5(key: &str, buckets: u64) -> u64 {
    if buckets == 0 {
        return 0;
    }
    mix_5(&[key.as_bytes()]) % buckets
}
