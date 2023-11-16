use crate::error_types::SubListErrors;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
};

type MessageId = usize;

pub struct SubscriptionManager {
    identifier_map: HashMap<MessageId, HashSet<SocketAddr>>,
}

impl SubscriptionManager {
    ///
    /// Create new SubscriptionManager to be used by the program
    ///
    pub fn new() -> Self {
        SubscriptionManager {
            identifier_map: HashMap::new(),
        }
    }

    ///
    /// Add a new subscription to the SubscriptionManager
    ///
    pub fn add_subscription(
        &mut self,
        message_id: MessageId,
        process: SocketAddr,
    ) -> Result<(), SubListErrors> {
        if let Some(values) = self.identifier_map.get_mut(&message_id) {
            let result = values.insert(process);
            if !result {
                Err(SubListErrors::SubscriptionAlreadyPresent)
            } else {
                Ok(())
            }
        } else {
            let mut set = HashSet::new();
            set.insert(process);
            self.identifier_map.insert(message_id, set);
            Ok(())
        }
    }

    ///
    /// Retrieves the subscribers to the provided message ID
    ///
    pub fn get_subscribers(
        &self,
        message_id: MessageId,
    ) -> Result<Vec<&SocketAddr>, SubListErrors> {
        match self.identifier_map.get(&message_id) {
            Some(sub) => Ok(sub.iter().collect()),
            None => Err(SubListErrors::NoSubscriptionFound),
        }
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use super::*;

    /// Verifies that the SubscriptionManager can be built
    #[test]
    fn test_can_build() {
        let manager = SubscriptionManager::new();
        assert_eq!(manager.identifier_map.len(), 0);
    }

    /// Verfies that subscriptions can be added to the manager
    #[test]
    fn test_can_add_sub() {
        let mut manager = SubscriptionManager::new();
        let message_id = 1;
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
        let result = manager.add_subscription(message_id, process);

        assert!(result.unwrap() == ());
        assert_eq!(manager.identifier_map.len(), 1);
    }

    /// Verifiest that the manager can add 2 addresses to a single ID
    #[test]
    fn test_can_add_two_sub() {
        let mut manager = SubscriptionManager::new();
        let message_id = 1;
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
        let result = manager.add_subscription(message_id, process);
        assert!(result.is_ok());
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8081));
        let result = manager.add_subscription(message_id, process);

        assert!(result.is_ok());
        assert_eq!(manager.identifier_map.get(&message_id).unwrap().len(), 2);
    }

    /// Verifies that an error is returned when a duplicate subscriber is attempted to be inserted
    #[test]
    fn test_error_duplicate_sub() {
        let mut manager = SubscriptionManager::new();
        let message_id = 1;
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
        let result = manager.add_subscription(message_id, process);
        assert!(result.is_ok());
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
        let result = manager.add_subscription(message_id, process);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            SubListErrors::SubscriptionAlreadyPresent
        );
        assert_eq!(manager.identifier_map.get(&message_id).unwrap().len(), 1);
    }

    /// Verifies that the subscribers can be retrieved from the manager
    #[test]
    fn test_get_subscriber() {
        let mut manager = SubscriptionManager::new();
        let message_id = 1;
        let process = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080));
        let result = manager.add_subscription(message_id, process);
        assert!(result.is_ok());

        let result = manager.get_subscribers(message_id);
        assert!(result.is_ok());
        assert!(result.unwrap().contains(&&process));
    }

    /// Verifies that an error is returned when get_subscribers is called with an ID with no subscribers
    #[test]
    fn test_get_subscriber_not_present() {
        let list = SubscriptionManager::new();
        let message_id = 1;
        let result = list.get_subscribers(message_id);
        if let Err(error) = result {
            assert_eq!(
                error,
                SubListErrors::NoSubscriptionFound,
                "The wrong Error value was returned"
            );
        } else {
            assert!(false);
        }
    }
}
