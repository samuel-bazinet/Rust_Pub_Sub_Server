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
                    todo!("Deal with error")
                }
            } else {
                todo!("Make error for invalid conversion")
            }
        }
    } else {
        if let Ok(message_id_bytes) = message[0..8].try_into() {
            if let Ok(subscribers) =
                sub_manager.get_subscribers(usize::from_le_bytes(message_id_bytes))
            {
                let udp_sender = UdpSocket::bind("0.0.0.0:0").unwrap();
                for subscriber in subscribers {
                    let result = udp_sender.send_to(&message[8..], subscriber);
                }
            }
        }
    }
}
