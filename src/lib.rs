mod configs;
mod constants;
mod error_types;
mod sub_list;

use std::net::UdpSocket;

pub use configs::setup_server;

///
/// Starts listening to the configured port.
///
pub fn start_listening(config: configs::PubSubConfigs) -> ! {
    if let Ok(udp_socket) = UdpSocket::bind(config.addr) {
        let mut buffer = [0u8;constants::BUFFER_SIZE];
        loop {
            if let Ok((size, src)) = udp_socket.recv_from(&mut buffer) {
                let partial_buf = &buffer[..size];
                todo!("Make function to parse bytes and determine if message or new subscriptions")
            } else {
                todo!("Replace with error enum for unable to receive")
            }
        }
    } else {
        todo!("Replace with error enum for unable to bind")
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_start_listening() {
        let config = setup_server(constants::test_resource_path("test_valid_config.json").as_str()).unwrap();
        start_listening(config);
    }
}
