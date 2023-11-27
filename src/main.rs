use pub_sub_server::{setup_server, start_listening};

fn main() {
    let config = setup_server("./configs/configs.json").unwrap();
    start_listening(config).unwrap();
}
