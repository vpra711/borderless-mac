mod constants;
mod settings;
mod structures;
mod syscalls;

use std::{io::Read, net::TcpListener};

use constants::*;
use settings::*;
use structures::*;
use syscalls::*;
use aes::{Aes256, cipher::{KeyInit, generic_array::GenericArray}};

fn main() {
    let clipboard_tcp_port = format!("127.0.0.1:{}", TCP_PORT_CLIPBOARD);
    let listener = TcpListener::bind(&clipboard_tcp_port);
    let mut from_remote = [0; 64];
    let mut config = Config::default();
    initial_setup(&mut config);

    if let Ok(tcp) = &listener {
        for stream in tcp.incoming() {
            loop {
                from_remote = [0; 64];
                if let Ok(size) = stream.as_ref().unwrap().read(&mut from_remote) && size != 0 {
                    println!("{}", generate_random_key());
                    println!("{}", String::from_utf8(from_remote.to_vec().clone()).unwrap());
                }
            }
        }
    } else {
        eprintln!("cannot bind to {}", &clipboard_tcp_port);
    }
}

fn initial_setup(config: &mut Config) {
    configure_environment(config);
    // init_encryption();
    let screen_size = syscalls::get_screen_size();
    println!("{:?}", screen_size);
    let username = syscalls::get_user_name();
    println!("{:?}", username);
}

fn init_encryption(config: &Config) {
    let cipher = Aes256::new_from_slice(config.my_key.as_bytes());
    
    
}

fn configure_environment(config: &mut Config) {
    config.user_name = get_user_name().0;
    config.my_key = generate_random_key();
    config.key_generated = true;
    config.machine_name = setup_machine_name_and_id();
}

fn generate_random_key() -> String {
    let mut key = String::new();
    while key.len() < PASSWD_LENGTH {
        let random: u8 = rand::random();
        let index = usize::from(random) % POSSIBLE_PASSWD_CHARS.len();
        let selected_char = POSSIBLE_PASSWD_CHARS.chars().nth(index).unwrap();
        key.push(selected_char);
    }
    key
}

// fn generate_iv() -> [u8; 16]{
//     let mut iv: [u8; 16] = [0; 16];
//     for x in iv.iter_mut() {
//         *x =  rand::random();
//     }
//     iv
// }

fn setup_machine_name_and_id() -> String {
    get_host_name()
}
