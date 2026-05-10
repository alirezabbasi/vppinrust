//! Interop boundary contracts between legacy C VPP modules and Rust rewrite modules.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PacketStatsInput {
    pub module: String,
    pub packets_in: u64,
    pub packets_out: u64,
    pub drops: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PacketStatsOutput {
    pub module: String,
    pub forwarded: u64,
    pub dropped: u64,
    pub drop_ratio_bps: u32,
}

pub trait PacketStatsAdapter {
    fn evaluate(&self, input: &PacketStatsInput) -> PacketStatsOutput;
}

/// Reference implementation used by the conformance harness.
#[derive(Debug, Default)]
pub struct RustPacketStatsAdapter;

impl PacketStatsAdapter for RustPacketStatsAdapter {
    fn evaluate(&self, input: &PacketStatsInput) -> PacketStatsOutput {
        let forwarded = input.packets_out;
        let dropped = input.drops;
        let total = input.packets_in.max(1);
        let drop_ratio_bps_u64 = dropped.saturating_mul(10_000) / total;
        let drop_ratio_bps = u32::try_from(drop_ratio_bps_u64).unwrap_or(u32::MAX);

        PacketStatsOutput { module: input.module.clone(), forwarded, dropped, drop_ratio_bps }
    }
}

#[cfg(test)]
mod tests {
    use super::{PacketStatsAdapter, PacketStatsInput, RustPacketStatsAdapter};

    #[test]
    fn adapter_computes_drop_ratio() {
        let adapter = RustPacketStatsAdapter;
        let input = PacketStatsInput {
            module: String::from("sample"),
            packets_in: 1000,
            packets_out: 995,
            drops: 5,
        };

        let out = adapter.evaluate(&input);
        assert_eq!(out.drop_ratio_bps, 50);
    }
}
