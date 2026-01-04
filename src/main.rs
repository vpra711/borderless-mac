#![allow(unused)]
mod structures;
mod network;
mod hashing;
mod syscalls;

use structures::*;
use network::*;
use hashing::Hasher;

fn main() {
    let mut config = Config::default();
    let mut hasher = Hasher::from(String::from("asdfasdfasdfasdf"));
    start_listening(&mut config, &mut hasher);
}

/*
flow of the application
    get username
    get encrypted passwd
    get screen configuration
    initialize the package monitor for sent and received packages
    setup machines and ids
        initialize machine pool
        learn machine
        try and update machine id
        update machine pool string setting
    initialize encryption
        create aes256 hasher
 
network stuff
    receiving 64 bytes of data at once, which mean data being sent is the same
*/
