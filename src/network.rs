// network related
use crate::structures::*;
use crate::hashing::Hasher;
use crate::syscalls::*;

use std::io::Read;
use std::net::Ipv4Addr;
use std::net::{
    TcpListener,
    TcpStream,
    SocketAddr
};

use std::ptr::slice_from_raw_parts;

pub fn start_listening(config: &mut Config, hasher: &Hasher) {
    config.user_name = get_user_name();
    config.package_sent = PackageMonitor::default();
    config.package_received = PackageMonitor::default();
    config.package_id = 0;

    // TODO: introduce multithreading to listen for both threads
    let addr = [
        SocketAddr::from(([0, 0, 0, 0], 15101)),
        SocketAddr::from(([0, 0, 0, 0], 15100)),
    ];

    let listener = TcpListener::bind(&addr[..]);
    let Ok(tcp_streams) = &listener else {
        dbg!("unable to bind to tcp listener");
        return;
    };
    
    dbg!("listening on");
    for mut tcp_stream in &mut tcp_streams.incoming() {
        let Ok(stream) = &mut tcp_stream else {
            dbg!("new tcp connection request rejected.");
            return;
        };
        handle_tcp_stream(stream, config, &hasher);
    }
}

pub fn handle_tcp_stream(stream: &mut TcpStream, config: &mut Config, hasher: &Hasher) {
    dbg!("new tcp stream accepted, receiving data");
    let mut received_bytes: [u8; 64];
    loop {
        received_bytes = [0; 64];
        match stream.read(&mut received_bytes) {
            Ok(bytes_read) => {
                println!("bytes read: {}", bytes_read);
            },
            Err(error) => {
                dbg!(error);
                std::process::exit(1);
            }
        }

        let mut decrypted_data = hasher.decrypt_data(received_bytes);
        println!("decrypted data: {:?}", decrypted_data);

        hasher.verify_and_update_package(&mut decrypted_data.as_chunks_mut::<32>().0[0]);
    }
}
