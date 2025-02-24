use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum SubListErrors {
    /// This subscription was already present in the list
    SubscriptionAlreadyPresent,
    /// The message ID is not present in the list
    NoSubscriptionFound,
}

impl Error for SubListErrors {}

impl Display for SubListErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubListErrors::SubscriptionAlreadyPresent => {
                write!(f, "The subscription was already added to the list.")
            }
            SubListErrors::NoSubscriptionFound => write!(
                f,
                "The provided message ID did not match any known subscription."
            ),
        }
    }
}
