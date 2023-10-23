use std::net::SocketAddr;

pub struct PubSubConfigs {
    pub addr: SocketAddr,
}

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

    use super::*;

    /// Verifies that the PubSubConfigs can be built
    #[test]
    fn test_configs_build() {
        let _ = PubSubConfigs {addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080))};
        assert!(true);
    }
}