//! Core primitives for the Rust VPP rewrite.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketBufferId(pub u32);

impl PacketBufferId {
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::PacketBufferId;

    #[test]
    fn packet_buffer_id_zero_is_invalid() {
        assert!(!PacketBufferId(0).is_valid());
    }

    #[test]
    fn packet_buffer_id_non_zero_is_valid() {
        assert!(PacketBufferId(7).is_valid());
    }
}
