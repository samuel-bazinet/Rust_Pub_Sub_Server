use crate::error_types::SubListErrors;

use std::collections::{HashMap, HashSet};

pub struct SubscriptionManager {
    identifier_map: HashMap<usize, HashSet<&'static str>>
}

impl SubscriptionManager {
    /// Create new SubscriptionManager to be used by the program
    pub fn new() -> Self {
        SubscriptionManager { identifier_map: HashMap::new() }
    }

    /// Add a new subscription to the SubscriptionManager
    pub fn add_subscription(&mut self, message_id: usize, process: &'static str) -> Result<(), SubListErrors> {

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

    /// Retrieves the subscribers to the provided message ID
    pub fn get_subscribers(& self, message_id: usize) -> Result<&HashSet<&'static str>, SubListErrors> {
        match self.identifier_map.get(&message_id) {
            Some(sub) => Ok(sub),
            None => Err(SubListErrors::NoSubscriptionFound)
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
    use super::*;

    #[test]
    fn test_can_build() {
        let list = SubscriptionManager::new();
        assert_eq!(list.identifier_map.len(), 0);
    }

    #[test]
    fn test_can_add_sub() {
        let mut list = SubscriptionManager::new();
        let message_id = 1;
        let process = "test";
        let result = list.add_subscription(message_id, process);

        assert!(result.unwrap() == ());
        assert_eq!(list.identifier_map.len(), 1);
    }
    
    #[test]
    fn test_can_add_two_sub() {
        let mut list = SubscriptionManager::new();
        let message_id = 1;
        let process = "test";
        let result = list.add_subscription(message_id, process);
        assert!(result.is_ok());
        let process = "test2";
        let result = list.add_subscription(message_id, process);

        assert!(result.is_ok());
        assert_eq!(list.identifier_map.get(&message_id).unwrap().len(), 2);
    }

    #[test]
    fn test_error_duplicate_sub() {
        let mut list = SubscriptionManager::new();
        let message_id = 1;
        let process = "test";
        let result = list.add_subscription(message_id, process);
        assert!(result.is_ok());
        let process = "test";
        let result = list.add_subscription(message_id, process);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), SubListErrors::SubscriptionAlreadyPresent);
        assert_eq!(list.identifier_map.get(&message_id).unwrap().len(), 1);
    }

    #[test]
    fn test_get_subscriber() {
        let mut list = SubscriptionManager::new();
        let message_id = 1;
        let process = "test";
        let result = list.add_subscription(message_id, process);
        assert!(result.is_ok());

        let result = list.get_subscribers(message_id);
        assert!(result.is_ok());
        assert!(result.unwrap().contains(process));
    }

    #[test]
    fn test_get_subscriber_not_present() {
        let list = SubscriptionManager::new();
        let message_id = 1;
        let result = list.get_subscribers(message_id);
        if let Err(error) = result {
            assert_eq!(error, SubListErrors::NoSubscriptionFound, "The wrong Error value was returned");
        } else {
            assert!(false);
        }
    }
}