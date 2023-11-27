# Rust Pub-Sub Server

This is a pub-sub server made with the intention of being simple to use.

It is meant to be used as a library, but if no wrapping is necessary it can be downloaded and ran as an executable.

This package depends on the [rpss_util](https://github.com/samuel-bazinet/rpss_utils) repo; this repo also holds functions that are useful to interact with the server from a client (WIP). 

## How to use as a library

1. Include the repo as a dependency of your Rust project.

2. Use the `setup_server(config_file)` function to generate the configs to be used by the server (See `./configs/configs.json` as an example for a config file).

3. Use the `start_listening(config)` function to start listening.

4. Running your program should now have it work as a Pub-Sub Server.

## How to use the executable

1. Download the repo to a known location.

2. Set the desired IP address to host the server in `./configs/configs.json`.

3. From the command line in the root of the downloaded repo:

    1. Run `cargo run --release`.
        
        1. If Rust is not installed, download [here](https://www.rust-lang.org/learn/get-started).
    
4. Server should be running.

## How to access from the client

1. From your client:

    1. Use the byte sequence `[0, 0, 0, 0, 0, 0, 0, 0]` as the first 8 bytes of a UDP payload to the server followed by a series of 8 byte long "message IDs" to add the client as a subscriber to those messages.

        1. Ex: `[0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1]` will make the client a subscriber to the messages with ID `0x0807060504030201` and `0x0102030405060708` (little endian is used) 

    2. When another client wants to send a message the client mentioned above, the first 8 bytes of the message will be the message ID (in little endian) followed by the content of the message.

        1. Ex: `[1, 2, 3, 4, 5, 6, 7, 8, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]` sent from a client to the server will have `[72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]` be sent to any clients subscribed to `[1, 2, 3, 4, 5, 6, 7, 8]`.


**Important:**  The message ID will be stripped of the message by the server.
