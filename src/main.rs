use pub_sub_server::{setup_server, start_listening};

fn main() {
    let config = setup_server("./test_resources/test_valid_config.json").unwrap();
    start_listening(config).unwrap();
}
