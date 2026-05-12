#![allow(unused)]
mod structures;
mod network;
mod hashing;
mod syscalls;
mod logger;
mod threadpool;

use std::thread;
use std::sync::*;
use std::cell::*;
use std::rc::Rc;

use structures::*;
use network::*;
use syscalls::*;
use hashing::Hasher;
use threadpool::*;

use std::net::SocketAddr;

static STATS: LazyLock<Arc<RwLock<Stats>>> = LazyLock::new(|| Arc::new(RwLock::new(Stats::default())));
static CONFIG: LazyLock<Arc<RwLock<Config>>> = LazyLock::new(|| Arc::new(RwLock::new(Config::default())));
static MATRIX: LazyLock<Arc<RwLock<Vec<MachineInfo>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));
static THREADPOOL: LazyLock<Arc<RwLock<Threadpool>>> = LazyLock::new(|| Arc::new(RwLock::new(Threadpool::new(10))));

fn main() {
    configure();
    start_listening();
    loop {}
}

fn configure() {
    if let Ok(mut config) = CONFIG.write() {
        config.user_name = get_user_name();
        config.machine_id = rand::random();
        config.key = String::from("mnopmnopmnopmnop");
    }
}
