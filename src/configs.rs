use std::{fs::read_to_string, net::SocketAddr, str::FromStr};

#[cfg(test)]
use crate::constants::test_resource_path;

use crate::error_types::SetupErrors;

use serde_json::Value;

pub struct PubSubConfigs {
    pub addr: SocketAddr,
}

///
/// Sets up the server. Reads a JSON file where the app settings are defined.
///
///  ## Example:
/// ```no_run
/// # use pub_sub_server::*;
/// let config = setup_server("configs/configs.json").unwrap();
/// start_listening(config);
/// ```
/// 
pub fn setup_server(config_file: &str) -> Result<PubSubConfigs, SetupErrors> {
    if let Ok(file_content) = read_to_string(config_file) {
        get_config_from_file_string(file_content)
    } else {
        Err(SetupErrors::CannotFindConfigFile)
    }
}

fn get_config_from_file_string(file_content: String) -> Result<PubSubConfigs, SetupErrors> {
    if let Ok(config) = serde_json::from_str::<Value>(&file_content) {
        get_socket_address_from_json(config)
    } else {
        Err(SetupErrors::CannotReadConfigFile)
    }
}

fn get_socket_address_from_json(config: Value) -> Result<PubSubConfigs, SetupErrors> {
    if let Some(socket_string) = config["addr"].as_str() {
        get_socket_address_from_str(socket_string)
    } else {
        Err(SetupErrors::InvalidConfigFileKey)
    }
}

fn get_socket_address_from_str(socket_string: &str) -> Result<PubSubConfigs, SetupErrors> {
    if let Ok(socket_address) = SocketAddr::from_str(socket_string) {
        Ok(PubSubConfigs {
            addr: socket_address,
        })
    } else {
        Err(SetupErrors::InvalidSocketAddress)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use super::*;

    /// Verifies that the PubSubConfigs can be built
    #[test]
    fn test_configs_build() {
        let _ = PubSubConfigs {
            addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
        };
        assert!(true);
    }

    #[test]
    fn test_setup_server_returns_configs() {
        let result = setup_server(test_resource_path("test_valid_config.json").as_str());
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_server_returns_invalid_socket() {
        let result = setup_server(test_resource_path("test_invalid_config_socket.json").as_str());
        if let Err(err_val) = result {
            assert_eq!(
                err_val,
                SetupErrors::InvalidSocketAddress,
                "The wrong enum value was used"
            )
        } else {
            assert!(false, "The function did not fail as expected")
        }
    }

    #[test]
    fn test_setup_server_returns_invalid_config_key() {
        let result = setup_server(test_resource_path("test_invalid_config_key.json").as_str());
        if let Err(err_val) = result {
            assert_eq!(
                err_val,
                SetupErrors::InvalidConfigFileKey,
                "The wrong enum value was used"
            )
        } else {
            assert!(false, "The function did not fail as expected")
        }
    }

    #[test]
    fn test_setup_server_returns_invalid_config_format() {
        let result = setup_server(test_resource_path("test_invalid_config_format").as_str());
        if let Err(err_val) = result {
            assert_eq!(
                err_val,
                SetupErrors::CannotReadConfigFile,
                "The wrong enum value was used"
            )
        } else {
            assert!(false, "The function did not fail as expected")
        }
    }

    #[test]
    fn test_setup_server_returns_invalid_config_file() {
        let result = setup_server(test_resource_path("").as_str());
        if let Err(err_val) = result {
            assert_eq!(
                err_val,
                SetupErrors::CannotFindConfigFile,
                "The wrong enum value was used"
            )
        } else {
            assert!(false, "The function did not fail as expected")
        }
    }
}
