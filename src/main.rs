mod constants;
mod settings;
mod structures;
mod syscalls;
mod network;
mod hashing;

use structures::*;
use network::*;
use hashing::*;

fn main() {
    let mut config = Config::default();
    init_encryption(&mut config);
    start_listening(&mut config);
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
