mod configs;
mod constants;
mod error_types;
mod message_processor;
mod sub_list;

use std::{
    net::UdpSocket,
    sync::{Arc, Mutex},
};

pub use configs::setup_server;
use error_types::ListeningErrors;
use sub_list::SubscriptionManager;

///
/// Starts listening to the configured port.
/// 
/// ## Example:
/// ```no_run
/// # use pub_sub_server::*;
/// let config = setup_server("configs/configs.json").unwrap();
/// start_listening(config);
/// ```
///
pub fn start_listening(config: configs::PubSubConfigs) -> Result<(), ListeningErrors> {
    if let Ok(udp_socket) = UdpSocket::bind(config.addr) {
        let mut buffer = [0u8; constants::BUFFER_SIZE];
        let subscription_manager = Arc::new(Mutex::new(SubscriptionManager::new()));
        loop {
            if let Ok((size, src)) = udp_socket.recv_from(&mut buffer) {
                let partial_buf = &buffer[..size];
                message_processor::process_message(
                    partial_buf,
                    src,
                    &mut subscription_manager.lock().unwrap(),
                );
            } else {
                return Err(ListeningErrors::UnableToReceive);
            }
        }
    } else {
        return Err(ListeningErrors::UnableToBind);
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};

    use super::*;

    #[test]
    fn start_listening_unbindable() {
        let config = configs::PubSubConfigs{addr: SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1))};
        let result = start_listening(config);
        if let Err(e) = result {
            assert_eq!(e, ListeningErrors::UnableToBind, "The wrong error was returned");
        } else {
            assert!(false, "Code was meant to return an error")
        }
    }

    #[test]
    fn start_listening_cant_receive() {
        
    }
}
