//! Control-plane state models.
use vpp_runtime as _;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceConfig {
    pub name: String,
    pub admin_up: bool,
}

impl InterfaceConfig {
    #[must_use]
    pub fn validate(&self) -> bool {
        !self.name.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::InterfaceConfig;

    #[test]
    fn config_name_must_not_be_empty() {
        let cfg = InterfaceConfig { name: String::new(), admin_up: true };
        assert!(!cfg.validate());
    }
}
