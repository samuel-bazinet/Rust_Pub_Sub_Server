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
        if let Ok(config) = serde_json::from_str::<Value>(&file_content) {
            if let Some(socket_string) = config["addr"].as_str() {
                if let Ok(socket_address) = SocketAddr::from_str(socket_string) {
                    Ok(PubSubConfigs {
                        addr: socket_address,
                    })
                } else {
                    Err(SetupErrors::InvalidSocketAddress)
                }
            } else {
                Err(SetupErrors::InvalidConfigFileFormat)
            }
        } else {
            Err(SetupErrors::CannotReadConfigFile)
        }
    } else {
        Err(SetupErrors::CannotFindConfigFile)
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
