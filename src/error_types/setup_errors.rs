use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum SetupErrors {
    CannotReadConfigFile,
    CannotFindConfigFile,
    InvalidConfigFileKey,
    InvalidSocketAddress,
}

impl Error for SetupErrors {}

impl Display for SetupErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupErrors::CannotReadConfigFile => 
                write!(f, "The config file cannot be read ."),
            SetupErrors::CannotFindConfigFile => 
                write!(f, "The path to the condifg file cannot be found."),
            SetupErrors::InvalidConfigFileKey => 
                write!(f, "The config file does not have the right keys."),
            SetupErrors::InvalidSocketAddress =>
                write!(f, "The address in the config file is not formatted properly.")
        }
    }
}