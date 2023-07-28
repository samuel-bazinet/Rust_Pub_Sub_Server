use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum SubListErrors {
    SubscriptionAlreadyPresent
}

impl Error for SubListErrors {
    
}

impl Display for SubListErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubListErrors::SubscriptionAlreadyPresent => write!(f, "The subscription was already added to the list")
        }
    }
}