//! Analytics pack 2 for Disputes (seed=47).
use crate::model::DisputesRecord;
use rust_decimal::Decimal;

pub struct DisputesAnalytics2;

impl DisputesAnalytics2 {
    pub fn metric_2_0(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(47)
            .wrapping_add(rec.attempt as i64 * 2);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_1(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(60)
            .wrapping_add(rec.attempt as i64 * 3);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_2(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(73)
            .wrapping_add(rec.attempt as i64 * 4);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_3(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(86)
            .wrapping_add(rec.attempt as i64 * 5);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_4(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(99)
            .wrapping_add(rec.attempt as i64 * 6);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_5(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(112)
            .wrapping_add(rec.attempt as i64 * 7);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_6(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(125)
            .wrapping_add(rec.attempt as i64 * 8);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_7(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(138)
            .wrapping_add(rec.attempt as i64 * 9);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_8(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(151)
            .wrapping_add(rec.attempt as i64 * 10);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_9(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(164)
            .wrapping_add(rec.attempt as i64 * 11);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(12);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_10(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(177)
            .wrapping_add(rec.attempt as i64 * 12);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(13);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_11(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(190)
            .wrapping_add(rec.attempt as i64 * 13);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(14);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_12(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(203)
            .wrapping_add(rec.attempt as i64 * 14);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(15);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_13(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(216)
            .wrapping_add(rec.attempt as i64 * 15);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(16);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_14(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(229)
            .wrapping_add(rec.attempt as i64 * 16);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(17);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_15(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(242)
            .wrapping_add(rec.attempt as i64 * 17);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(18);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_16(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(255)
            .wrapping_add(rec.attempt as i64 * 18);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(19);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_17(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(268)
            .wrapping_add(rec.attempt as i64 * 19);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(20);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_18(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(281)
            .wrapping_add(rec.attempt as i64 * 20);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(21);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_19(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(294)
            .wrapping_add(rec.attempt as i64 * 21);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(22);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_20(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(307)
            .wrapping_add(rec.attempt as i64 * 22);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(23);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_21(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(320)
            .wrapping_add(rec.attempt as i64 * 23);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(24);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_22(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(333)
            .wrapping_add(rec.attempt as i64 * 24);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(25);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_23(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(346)
            .wrapping_add(rec.attempt as i64 * 25);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(26);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_24(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(359)
            .wrapping_add(rec.attempt as i64 * 26);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(27);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_25(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(372)
            .wrapping_add(rec.attempt as i64 * 27);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(28);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_26(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(385)
            .wrapping_add(rec.attempt as i64 * 28);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(29);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_27(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(398)
            .wrapping_add(rec.attempt as i64 * 29);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(30);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_28(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(411)
            .wrapping_add(rec.attempt as i64 * 30);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(31);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_29(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(424)
            .wrapping_add(rec.attempt as i64 * 31);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(1);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_30(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(437)
            .wrapping_add(rec.attempt as i64 * 32);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_31(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(450)
            .wrapping_add(rec.attempt as i64 * 33);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_32(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(463)
            .wrapping_add(rec.attempt as i64 * 34);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_33(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(476)
            .wrapping_add(rec.attempt as i64 * 35);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_34(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(489)
            .wrapping_add(rec.attempt as i64 * 36);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_35(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(502)
            .wrapping_add(rec.attempt as i64 * 37);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_36(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(515)
            .wrapping_add(rec.attempt as i64 * 38);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_37(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(528)
            .wrapping_add(rec.attempt as i64 * 39);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_38(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(541)
            .wrapping_add(rec.attempt as i64 * 40);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_39(rec: &DisputesRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(554)
            .wrapping_add(rec.attempt as i64 * 41);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn feature_vector(rec: &DisputesRecord) -> Vec<i64> {
        let mut out = Vec::with_capacity(40);
        out.push(Self::metric_2_0(rec));
        out.push(Self::metric_2_1(rec));
        out.push(Self::metric_2_2(rec));
        out.push(Self::metric_2_3(rec));
        out.push(Self::metric_2_4(rec));
        out.push(Self::metric_2_5(rec));
        out.push(Self::metric_2_6(rec));
        out.push(Self::metric_2_7(rec));
        out.push(Self::metric_2_8(rec));
        out.push(Self::metric_2_9(rec));
        out.push(Self::metric_2_10(rec));
        out.push(Self::metric_2_11(rec));
        out.push(Self::metric_2_12(rec));
        out.push(Self::metric_2_13(rec));
        out.push(Self::metric_2_14(rec));
        out.push(Self::metric_2_15(rec));
        out.push(Self::metric_2_16(rec));
        out.push(Self::metric_2_17(rec));
        out.push(Self::metric_2_18(rec));
        out.push(Self::metric_2_19(rec));
        out.push(Self::metric_2_20(rec));
        out.push(Self::metric_2_21(rec));
        out.push(Self::metric_2_22(rec));
        out.push(Self::metric_2_23(rec));
        out.push(Self::metric_2_24(rec));
        out.push(Self::metric_2_25(rec));
        out.push(Self::metric_2_26(rec));
        out.push(Self::metric_2_27(rec));
        out.push(Self::metric_2_28(rec));
        out.push(Self::metric_2_29(rec));
        out.push(Self::metric_2_30(rec));
        out.push(Self::metric_2_31(rec));
        out.push(Self::metric_2_32(rec));
        out.push(Self::metric_2_33(rec));
        out.push(Self::metric_2_34(rec));
        out.push(Self::metric_2_35(rec));
        out.push(Self::metric_2_36(rec));
        out.push(Self::metric_2_37(rec));
        out.push(Self::metric_2_38(rec));
        out.push(Self::metric_2_39(rec));
        out
    }

    pub fn score_blend(rec: &DisputesRecord, weights: &[Decimal]) -> Decimal {
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

    pub fn anomaly_flags(rec: &DisputesRecord) -> Vec<&'static str> {
        let mut flags = Vec::new();
        let feats = Self::feature_vector(rec);
        let max = feats.iter().copied().max().unwrap_or(0).abs();
        if max > 10470 {
            flags.push("SPIKE");
        }
        if rec.attempt > 4 {
            flags.push("RETRY_HEAVY");
        }
        if rec.risk_flags.iter().any(|f| f.contains("BLOCK")) {
            flags.push("BLOCKED");
        }
        flags
    }
}
