// network related

use aes::Aes256;
use aes::cipher::BlockDecryptMut;
use aes::cipher::KeyInit;
use aes::cipher::generic_array::GenericArray;

use crate::constants::*;
use crate::structures::*;
use crate::syscalls::*;
use crate::hashing::*;

use std::io::Read;
use std::net::Ipv4Addr;
use std::net::{
    TcpListener,
    TcpStream,
    SocketAddr
};

pub fn start_listening(config: &mut Config) {
    config.user_name = get_user_name();
    config.package_sent = PackageMonitor::default();
    config.package_received = PackageMonitor::default();
    config.package_id = 0;

    // TODO: introduce multithreading to listen for both threads
    let addr = [
        SocketAddr::from(([0, 0, 0, 0], TCP_PORT_MESSAGE)),
        SocketAddr::from(([0, 0, 0, 0], TCP_PORT_CLIPBOARD)),
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
        handle_tcp_stream(stream);
    }
}

pub fn handle_tcp_stream(stream: &mut TcpStream) {
    dbg!("new tcp stream accepted, receiving data");
    let mut received_bytes: [u8; 64];
    let mut out_black: [u8; 64];
    loop {
        received_bytes = [0; 64];
        match stream.read(&mut received_bytes) {
            Ok(data) => {
                println!("{}", data);
            },
            Err(error) => {
                dbg!(error);
            }
        }

        let key = "asdfgfasdfgfasdf".to_string();
        let mut aes = Aes256::new_from_slice(&generate_secure_key(key));
        let Ok(aes) = &mut aes else {
            dbg!("problem in aes512 cipher generation");
            return;
        };

        let mut blocks: [GenericArray<u8, _>; 4] = [
            *GenericArray::from_slice(&received_bytes[0..16]),
            *GenericArray::from_slice(&received_bytes[16..32]),
            *GenericArray::from_slice(&received_bytes[32..48]),
            *GenericArray::from_slice(&received_bytes[48..64]),
        ];

        aes.decrypt_blocks_mut(&mut blocks);
        let result: Vec<u8> = blocks.iter().flat_map(|b| b.iter().copied()).collect();
        println!("result is: {:?}", result);
        let ptr: *const u8 = result.as_ptr();
        let package = Data::from(&result.as_chunks::<64>().0[0]).unwrap();
        package.print();
    }
}

pub fn send_package() {
	
}
