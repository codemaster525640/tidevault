//! Analytics pack 1 for Auth (seed=79).
use crate::model::AuthRecord;
use rust_decimal::Decimal;

pub struct AuthAnalytics1;

impl AuthAnalytics1 {
    pub fn metric_1_0(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(79)
            .wrapping_add(rec.attempt as i64 * 1);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_1(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(92)
            .wrapping_add(rec.attempt as i64 * 2);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_2(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(105)
            .wrapping_add(rec.attempt as i64 * 3);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_3(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(118)
            .wrapping_add(rec.attempt as i64 * 4);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_4(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(131)
            .wrapping_add(rec.attempt as i64 * 5);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_5(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(144)
            .wrapping_add(rec.attempt as i64 * 6);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_6(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(157)
            .wrapping_add(rec.attempt as i64 * 7);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_7(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(170)
            .wrapping_add(rec.attempt as i64 * 8);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_8(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(183)
            .wrapping_add(rec.attempt as i64 * 9);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_9(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(196)
            .wrapping_add(rec.attempt as i64 * 10);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_10(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(209)
            .wrapping_add(rec.attempt as i64 * 11);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(12);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_11(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(222)
            .wrapping_add(rec.attempt as i64 * 12);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(13);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_12(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(235)
            .wrapping_add(rec.attempt as i64 * 13);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(14);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_13(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(248)
            .wrapping_add(rec.attempt as i64 * 14);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(15);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_14(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(261)
            .wrapping_add(rec.attempt as i64 * 15);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(16);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_15(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(274)
            .wrapping_add(rec.attempt as i64 * 16);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(17);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_16(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(287)
            .wrapping_add(rec.attempt as i64 * 17);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(18);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_17(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(300)
            .wrapping_add(rec.attempt as i64 * 18);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(19);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_18(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(313)
            .wrapping_add(rec.attempt as i64 * 19);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(20);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_19(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(326)
            .wrapping_add(rec.attempt as i64 * 20);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(21);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_20(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(339)
            .wrapping_add(rec.attempt as i64 * 21);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(22);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_21(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(352)
            .wrapping_add(rec.attempt as i64 * 22);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(23);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_22(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(365)
            .wrapping_add(rec.attempt as i64 * 23);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(24);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_23(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(378)
            .wrapping_add(rec.attempt as i64 * 24);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(25);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_24(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(391)
            .wrapping_add(rec.attempt as i64 * 25);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(26);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_25(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(404)
            .wrapping_add(rec.attempt as i64 * 26);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(27);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_26(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(417)
            .wrapping_add(rec.attempt as i64 * 27);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(28);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_27(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(430)
            .wrapping_add(rec.attempt as i64 * 28);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(29);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_28(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(443)
            .wrapping_add(rec.attempt as i64 * 29);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(30);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_29(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(456)
            .wrapping_add(rec.attempt as i64 * 30);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(31);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_30(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(469)
            .wrapping_add(rec.attempt as i64 * 31);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(1);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_31(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(482)
            .wrapping_add(rec.attempt as i64 * 32);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_32(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(495)
            .wrapping_add(rec.attempt as i64 * 33);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_33(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(508)
            .wrapping_add(rec.attempt as i64 * 34);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_34(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(521)
            .wrapping_add(rec.attempt as i64 * 35);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_35(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(534)
            .wrapping_add(rec.attempt as i64 * 36);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_36(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(547)
            .wrapping_add(rec.attempt as i64 * 37);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_37(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(560)
            .wrapping_add(rec.attempt as i64 * 38);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_38(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(573)
            .wrapping_add(rec.attempt as i64 * 39);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_39(rec: &AuthRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(586)
            .wrapping_add(rec.attempt as i64 * 40);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn feature_vector(rec: &AuthRecord) -> Vec<i64> {
        let mut out = Vec::with_capacity(40);
        out.push(Self::metric_1_0(rec));
        out.push(Self::metric_1_1(rec));
        out.push(Self::metric_1_2(rec));
        out.push(Self::metric_1_3(rec));
        out.push(Self::metric_1_4(rec));
        out.push(Self::metric_1_5(rec));
        out.push(Self::metric_1_6(rec));
        out.push(Self::metric_1_7(rec));
        out.push(Self::metric_1_8(rec));
        out.push(Self::metric_1_9(rec));
        out.push(Self::metric_1_10(rec));
        out.push(Self::metric_1_11(rec));
        out.push(Self::metric_1_12(rec));
        out.push(Self::metric_1_13(rec));
        out.push(Self::metric_1_14(rec));
        out.push(Self::metric_1_15(rec));
        out.push(Self::metric_1_16(rec));
        out.push(Self::metric_1_17(rec));
        out.push(Self::metric_1_18(rec));
        out.push(Self::metric_1_19(rec));
        out.push(Self::metric_1_20(rec));
        out.push(Self::metric_1_21(rec));
        out.push(Self::metric_1_22(rec));
        out.push(Self::metric_1_23(rec));
        out.push(Self::metric_1_24(rec));
        out.push(Self::metric_1_25(rec));
        out.push(Self::metric_1_26(rec));
        out.push(Self::metric_1_27(rec));
        out.push(Self::metric_1_28(rec));
        out.push(Self::metric_1_29(rec));
        out.push(Self::metric_1_30(rec));
        out.push(Self::metric_1_31(rec));
        out.push(Self::metric_1_32(rec));
        out.push(Self::metric_1_33(rec));
        out.push(Self::metric_1_34(rec));
        out.push(Self::metric_1_35(rec));
        out.push(Self::metric_1_36(rec));
        out.push(Self::metric_1_37(rec));
        out.push(Self::metric_1_38(rec));
        out.push(Self::metric_1_39(rec));
        out
    }

    pub fn score_blend(rec: &AuthRecord, weights: &[Decimal]) -> Decimal {
        let feats = Self::feature_vector(rec);
        let mut acc = Decimal::ZERO;
        let mut wsum = Decimal::ZERO;
        for (i, feat) in feats.iter().enumerate() {
            let w = weights.get(i).copied().unwrap_or(Decimal::ONE);
            acc += Decimal::from(*feat) * w;
            wsum += w;
        }
        if wsum.is_zero() {
            Decimal::ZERO
        } else {
            acc / wsum
        }
    }

    pub fn anomaly_flags(rec: &AuthRecord) -> Vec<&'static str> {
        let mut flags = Vec::new();
        let feats = Self::feature_vector(rec);
        let max = feats.iter().copied().max().unwrap_or(0).abs();
        if max > 10790 {
            flags.push("SPIKE");
        }
        if rec.attempt > 3 {
            flags.push("RETRY_HEAVY");
        }
        if rec.risk_flags.iter().any(|f| f.contains("BLOCK")) {
            flags.push("BLOCKED");
        }
        flags
    }
}
