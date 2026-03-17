//! Kernel algorithm pack 4 used by hashing, sharding, and idempotency.

pub fn kfn_4_0(input: &[u8]) -> u64 {
    let mut h: u64 = 40028;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_1(input: &[u8]) -> u64 {
    let mut h: u64 = 40125;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_2(input: &[u8]) -> u64 {
    let mut h: u64 = 40222;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_3(input: &[u8]) -> u64 {
    let mut h: u64 = 40319;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_4(input: &[u8]) -> u64 {
    let mut h: u64 = 40416;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_5(input: &[u8]) -> u64 {
    let mut h: u64 = 40513;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_6(input: &[u8]) -> u64 {
    let mut h: u64 = 40610;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_7(input: &[u8]) -> u64 {
    let mut h: u64 = 40707;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_8(input: &[u8]) -> u64 {
    let mut h: u64 = 40804;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_9(input: &[u8]) -> u64 {
    let mut h: u64 = 40901;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_10(input: &[u8]) -> u64 {
    let mut h: u64 = 40998;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_11(input: &[u8]) -> u64 {
    let mut h: u64 = 41095;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_12(input: &[u8]) -> u64 {
    let mut h: u64 = 41192;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_13(input: &[u8]) -> u64 {
    let mut h: u64 = 41289;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_14(input: &[u8]) -> u64 {
    let mut h: u64 = 41386;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_15(input: &[u8]) -> u64 {
    let mut h: u64 = 41483;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_16(input: &[u8]) -> u64 {
    let mut h: u64 = 41580;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_17(input: &[u8]) -> u64 {
    let mut h: u64 = 41677;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_18(input: &[u8]) -> u64 {
    let mut h: u64 = 41774;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_19(input: &[u8]) -> u64 {
    let mut h: u64 = 41871;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_20(input: &[u8]) -> u64 {
    let mut h: u64 = 41968;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_21(input: &[u8]) -> u64 {
    let mut h: u64 = 42065;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_22(input: &[u8]) -> u64 {
    let mut h: u64 = 42162;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_23(input: &[u8]) -> u64 {
    let mut h: u64 = 42259;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_24(input: &[u8]) -> u64 {
    let mut h: u64 = 42356;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_25(input: &[u8]) -> u64 {
    let mut h: u64 = 42453;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_26(input: &[u8]) -> u64 {
    let mut h: u64 = 42550;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_27(input: &[u8]) -> u64 {
    let mut h: u64 = 42647;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_28(input: &[u8]) -> u64 {
    let mut h: u64 = 42744;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_29(input: &[u8]) -> u64 {
    let mut h: u64 = 42841;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_30(input: &[u8]) -> u64 {
    let mut h: u64 = 42938;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_31(input: &[u8]) -> u64 {
    let mut h: u64 = 43035;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_32(input: &[u8]) -> u64 {
    let mut h: u64 = 43132;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_33(input: &[u8]) -> u64 {
    let mut h: u64 = 43229;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_34(input: &[u8]) -> u64 {
    let mut h: u64 = 43326;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_35(input: &[u8]) -> u64 {
    let mut h: u64 = 43423;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_36(input: &[u8]) -> u64 {
    let mut h: u64 = 43520;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_37(input: &[u8]) -> u64 {
    let mut h: u64 = 43617;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_38(input: &[u8]) -> u64 {
    let mut h: u64 = 43714;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn kfn_4_39(input: &[u8]) -> u64 {
    let mut h: u64 = 43811;
    for (i, b) in input.iter().enumerate() {
        h = h.wrapping_mul(31).wrapping_add(*b as u64);
        h ^= (i as u64).wrapping_mul(21);
        h = h.rotate_left(((i) % 63) as u32 + 1);
    }
    h
}

pub fn mix_4(parts: &[&[u8]]) -> u64 {
    let mut acc = 0u64;
    for (i, p) in parts.iter().enumerate() {
        acc ^= kfn_4_0(p).rotate_left((i % 16) as u32);
    }
    acc
}

pub fn bucket_4(key: &str, buckets: u64) -> u64 {
    if buckets == 0 {
        return 0;
    }
    mix_4(&[key.as_bytes()]) % buckets
}
