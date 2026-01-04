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

pub fn start_listening(config: &mut Config, hasher: &mut Hasher) {
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
        println!("unable to bind to tcp listener");
        return;
    };
    
    println!("listening on");
    for mut tcp_stream in &mut tcp_streams.incoming() {
        let Ok(stream) = &mut tcp_stream else {
            println!("new tcp connection request rejected.");
            return;
        };
        handle_tcp_stream(stream, hasher);
    }
}

pub fn handle_tcp_stream(stream: &mut TcpStream, hasher: &mut Hasher) {
    println!("new tcp stream accepted, receiving data");
    let mut received_bytes: [u8; 32];
    let mut ignore = [0u8; 16];
    stream.read_exact(&mut ignore);
    hasher.decrypt_data_16(&mut ignore);
    loop {
        received_bytes = [0; 32];
        match stream.read_exact(&mut received_bytes) {
            Ok(bytes_read) => {},
            Err(error) => {
                println!("{}", error);
                std::process::exit(1);
            }
        }
        hasher.decrypt_data_32(&mut received_bytes);
        hasher.verify_and_update_package(&mut received_bytes);
    }
}
