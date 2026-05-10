//! Runtime abstractions for node execution.

use vpp_core::PacketBufferId;

#[derive(Debug, Default)]
pub struct NodeBatch {
    packets: Vec<PacketBufferId>,
}

impl NodeBatch {
    #[must_use]
    pub fn len(&self) -> usize {
        self.packets.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    pub fn push(&mut self, id: PacketBufferId) {
        if id.is_valid() {
            self.packets.push(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NodeBatch;
    use vpp_core::PacketBufferId;

    #[test]
    fn invalid_packets_are_filtered() {
        let mut batch = NodeBatch::default();
        batch.push(PacketBufferId(0));
        batch.push(PacketBufferId(1));
        assert_eq!(batch.len(), 1);
    }
}
