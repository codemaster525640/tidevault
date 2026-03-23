//! Analytics pack 2 for Audit (seed=83).
use crate::model::AuditRecord;
use rust_decimal::Decimal;

pub struct AuditAnalytics2;

impl AuditAnalytics2 {
    pub fn metric_2_0(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(83)
            .wrapping_add(rec.attempt as i64 * 2);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_1(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(96)
            .wrapping_add(rec.attempt as i64 * 3);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_2(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(109)
            .wrapping_add(rec.attempt as i64 * 4);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_3(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(122)
            .wrapping_add(rec.attempt as i64 * 5);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_4(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(135)
            .wrapping_add(rec.attempt as i64 * 6);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_5(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(148)
            .wrapping_add(rec.attempt as i64 * 7);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_6(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(161)
            .wrapping_add(rec.attempt as i64 * 8);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_7(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(174)
            .wrapping_add(rec.attempt as i64 * 9);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_8(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(187)
            .wrapping_add(rec.attempt as i64 * 10);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_9(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(200)
            .wrapping_add(rec.attempt as i64 * 11);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(12);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_10(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(213)
            .wrapping_add(rec.attempt as i64 * 12);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(13);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_11(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(226)
            .wrapping_add(rec.attempt as i64 * 13);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(14);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_12(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(239)
            .wrapping_add(rec.attempt as i64 * 14);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(15);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_13(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(252)
            .wrapping_add(rec.attempt as i64 * 15);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(16);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_14(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(265)
            .wrapping_add(rec.attempt as i64 * 16);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(17);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_15(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(278)
            .wrapping_add(rec.attempt as i64 * 17);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(18);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_16(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(291)
            .wrapping_add(rec.attempt as i64 * 18);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(19);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_17(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(304)
            .wrapping_add(rec.attempt as i64 * 19);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(20);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_18(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(317)
            .wrapping_add(rec.attempt as i64 * 20);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(21);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_19(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(330)
            .wrapping_add(rec.attempt as i64 * 21);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(22);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_20(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(343)
            .wrapping_add(rec.attempt as i64 * 22);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(23);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_21(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(356)
            .wrapping_add(rec.attempt as i64 * 23);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(24);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_22(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(369)
            .wrapping_add(rec.attempt as i64 * 24);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(25);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_23(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(382)
            .wrapping_add(rec.attempt as i64 * 25);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(26);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_24(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(395)
            .wrapping_add(rec.attempt as i64 * 26);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(27);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_25(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(408)
            .wrapping_add(rec.attempt as i64 * 27);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(28);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_26(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(421)
            .wrapping_add(rec.attempt as i64 * 28);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(29);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_27(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(434)
            .wrapping_add(rec.attempt as i64 * 29);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(30);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_28(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(447)
            .wrapping_add(rec.attempt as i64 * 30);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(31);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_29(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(460)
            .wrapping_add(rec.attempt as i64 * 31);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(1);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_30(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(473)
            .wrapping_add(rec.attempt as i64 * 32);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(2);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_31(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(486)
            .wrapping_add(rec.attempt as i64 * 33);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(3);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_32(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(499)
            .wrapping_add(rec.attempt as i64 * 34);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(4);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_33(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(512)
            .wrapping_add(rec.attempt as i64 * 35);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(5);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_34(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(525)
            .wrapping_add(rec.attempt as i64 * 36);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(6);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_35(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(538)
            .wrapping_add(rec.attempt as i64 * 37);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(7);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 3);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_36(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(551)
            .wrapping_add(rec.attempt as i64 * 38);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(8);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 4);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_37(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(564)
            .wrapping_add(rec.attempt as i64 * 39);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(9);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 5);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_38(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(577)
            .wrapping_add(rec.attempt as i64 * 40);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(10);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 6);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn metric_2_39(rec: &AuditRecord) -> i64 {
        let minor = rec.amount.to_minor().unwrap_or(0);
        let mut h = (minor as i64)
            .wrapping_mul(590)
            .wrapping_add(rec.attempt as i64 * 41);
        h ^= (rec.priority as i64).wrapping_mul(19);
        h = h.rotate_left(11);
        h = h.wrapping_add(rec.risk_flags.len() as i64 * 7);
        h.clamp(-1_000_000_000, 1_000_000_000)
    }

    pub fn feature_vector(rec: &AuditRecord) -> Vec<i64> {
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

    pub fn score_blend(rec: &AuditRecord, weights: &[Decimal]) -> Decimal {
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

    pub fn anomaly_flags(rec: &AuditRecord) -> Vec<&'static str> {
        let mut flags = Vec::new();
        let feats = Self::feature_vector(rec);
        let max = feats.iter().copied().max().unwrap_or(0).abs();
        if max > 10830 {
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
