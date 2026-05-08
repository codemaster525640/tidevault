//! Analytics pack 1 for Reconcile (seed=41).
use crate::model::ReconcileRecord;
use rust_decimal::Decimal;

pub struct ReconcileAnalytics1;

impl ReconcileAnalytics1 {
    pub fn metric_1_0(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(41)
            .wrapping_add(rec.attempt as i64 * 1);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_1(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(54)
            .wrapping_add(rec.attempt as i64 * 2);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_2(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(67)
            .wrapping_add(rec.attempt as i64 * 3);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_3(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(80)
            .wrapping_add(rec.attempt as i64 * 4);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_4(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(93)
            .wrapping_add(rec.attempt as i64 * 5);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_5(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(106)
            .wrapping_add(rec.attempt as i64 * 6);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_6(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(119)
            .wrapping_add(rec.attempt as i64 * 7);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_7(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(132)
            .wrapping_add(rec.attempt as i64 * 8);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_8(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(145)
            .wrapping_add(rec.attempt as i64 * 9);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_9(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(158)
            .wrapping_add(rec.attempt as i64 * 10);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_10(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(171)
            .wrapping_add(rec.attempt as i64 * 11);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(12);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_11(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(184)
            .wrapping_add(rec.attempt as i64 * 12);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(13);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_12(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(197)
            .wrapping_add(rec.attempt as i64 * 13);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(14);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_13(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(210)
            .wrapping_add(rec.attempt as i64 * 14);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(15);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_14(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(223)
            .wrapping_add(rec.attempt as i64 * 15);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(16);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_15(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(236)
            .wrapping_add(rec.attempt as i64 * 16);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(17);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_16(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(249)
            .wrapping_add(rec.attempt as i64 * 17);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(18);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_17(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(262)
            .wrapping_add(rec.attempt as i64 * 18);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(19);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_18(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(275)
            .wrapping_add(rec.attempt as i64 * 19);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(20);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_19(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(288)
            .wrapping_add(rec.attempt as i64 * 20);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(21);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_20(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(301)
            .wrapping_add(rec.attempt as i64 * 21);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(22);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_21(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(314)
            .wrapping_add(rec.attempt as i64 * 22);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(23);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_22(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(327)
            .wrapping_add(rec.attempt as i64 * 23);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(24);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_23(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(340)
            .wrapping_add(rec.attempt as i64 * 24);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(25);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_24(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(353)
            .wrapping_add(rec.attempt as i64 * 25);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(26);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_25(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(366)
            .wrapping_add(rec.attempt as i64 * 26);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(27);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_26(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(379)
            .wrapping_add(rec.attempt as i64 * 27);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(28);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_27(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(392)
            .wrapping_add(rec.attempt as i64 * 28);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(29);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_28(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(405)
            .wrapping_add(rec.attempt as i64 * 29);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(30);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_29(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(418)
            .wrapping_add(rec.attempt as i64 * 30);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(31);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_30(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(431)
            .wrapping_add(rec.attempt as i64 * 31);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(1);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_31(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(444)
            .wrapping_add(rec.attempt as i64 * 32);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_32(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(457)
            .wrapping_add(rec.attempt as i64 * 33);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_33(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(470)
            .wrapping_add(rec.attempt as i64 * 34);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_34(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(483)
            .wrapping_add(rec.attempt as i64 * 35);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_35(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(496)
            .wrapping_add(rec.attempt as i64 * 36);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_36(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(509)
            .wrapping_add(rec.attempt as i64 * 37);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_37(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(522)
            .wrapping_add(rec.attempt as i64 * 38);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_38(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(535)
            .wrapping_add(rec.attempt as i64 * 39);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_1_39(rec: &ReconcileRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(548)
            .wrapping_add(rec.attempt as i64 * 40);
        h ^= (rec.priority as i64).wrapping_mul(18);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn feature_vector(rec: &ReconcileRecord) -> Vec<i64> {
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

    pub fn score_blend(rec: &ReconcileRecord, weights: &[Decimal]) -> Decimal {
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

    pub fn anomaly_flags(rec: &ReconcileRecord) -> Vec<&'static str> {
        let mut flags = Vec::new();
        let feats = Self::feature_vector(rec);
        let max = feats.iter().copied().max().unwrap_or(0).abs();
        if max > 10410 {
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
