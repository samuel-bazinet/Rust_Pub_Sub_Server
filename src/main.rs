use pub_sub_server::{setup_server, start_listening};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Started up the server");
    let config = setup_server("./configs/configs.json").unwrap();
    start_listening(config).unwrap();
}
