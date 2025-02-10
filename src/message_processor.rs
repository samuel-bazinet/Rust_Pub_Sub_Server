use rpss_utils::server_ops;

use std::net::{SocketAddr, UdpSocket};

use crate::sub_list::SubscriptionManager;

pub fn process_message(message: &[u8], src: SocketAddr, sub_manager: &mut SubscriptionManager) {
    if server_ops::is_update_subscriptions(&message[0..8]) {
        for message_id_bytes in message[8..].chunks(8) {
            if let Ok(message_id_bytes) = message_id_bytes.try_into() {
                if let Err(error) =
                    sub_manager.add_subscription(usize::from_le_bytes(message_id_bytes), src)
                {
                    log::warn!("Could not add the subscriber to the subscription list of {} with error {}", usize::from_le_bytes(message_id_bytes), error)
                }
            } else {
                log::warn!("Could not extract the message ID from the first 8 bytes (bytes: {:?})", message_id_bytes);
            }
        }
    } else if let Ok(message_id_bytes) = message[0..8].try_into() {
        if let Ok(subscribers) = sub_manager.get_subscribers(usize::from_le_bytes(message_id_bytes))
        {
            let udp_sender = UdpSocket::bind("0.0.0.0:0").unwrap();
            for subscriber in subscribers {
                log::debug!("Sending to {subscriber}");
                let result = udp_sender.send_to(&message[8..], subscriber);
                if let Err(error) = result {
                    log::warn!("Could not send the message to {subscriber} with error {}", error);
                }
            }
        }
    }
}
