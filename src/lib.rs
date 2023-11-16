mod configs;
mod constants;
mod error_types;
mod sub_list;
mod message_processor;

use std::{net::UdpSocket, sync::{Mutex, Arc}};

pub use configs::setup_server;
use error_types::ListeningErrors;
use sub_list::SubscriptionManager;

///
/// Starts listening to the configured port.
///
pub fn start_listening(config: configs::PubSubConfigs) -> Result<(), ListeningErrors> {
    let mut error;
    if let Ok(udp_socket) = UdpSocket::bind(config.addr) {
        let mut buffer = [0u8;constants::BUFFER_SIZE];
        let subscription_manager = Arc::new(Mutex::new(SubscriptionManager::new())); 
        loop {
            if let Ok((size, src)) = udp_socket.recv_from(&mut buffer) {
                let partial_buf = &buffer[..size];
                message_processor::process_message(partial_buf, src, &mut subscription_manager.lock().unwrap());
                todo!("Make function to parse bytes and determine if message or new subscriptions. Will need a thread pool.")
            } else {
                error = ListeningErrors::UnableToReceive;
                todo!("Add a logger to log the error then exit the program")
            }
        }
    } else {
        error = ListeningErrors::UnableToBind;
        todo!("Add a logger to log the error then exit the program")
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
