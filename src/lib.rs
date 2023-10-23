use std::{fs::read_to_string, net::SocketAddr, str::FromStr};

mod configs;
mod constants;
mod error_types;
mod sub_list;

use configs::PubSubConfigs;
use error_types::SetupErrors;

use serde_json::Value;

///
/// Sets up the server. Reads a JSON file where the app settings are defined.
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
        Err(SetupErrors::InvalidConfigFileFormat)
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

///
/// Starts listening to the configured port.
///
pub fn start_listening() -> ! {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_server_returns_configs() {
        let result = setup_server(constants::test_resource_path("test.json").as_str());
        assert!(result.is_ok());
    }
}
