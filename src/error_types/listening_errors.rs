use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum ListeningErrors {
    UnableToBind,
    UnableToReceive,
}

impl Error for ListeningErrors {}

impl Display for ListeningErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListeningErrors::UnableToBind => 
                write!(f, "The server is unable to bind to the IP Address"),
            ListeningErrors::UnableToReceive => 
                write!(f, "The server is unable to receive a packet")
        }
    }
}