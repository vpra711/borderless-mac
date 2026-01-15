#![allow(unused)]
mod structures;
mod network;
mod hashing;
mod syscalls;
mod logger;

use std::thread;
use std::sync::*;
use std::cell::*;
use std::rc::Rc;

use structures::*;
use network::*;
use syscalls::*;
use hashing::Hasher;

static STATS: LazyLock<Arc<RwLock<Stats>>> = LazyLock::new(|| Arc::new(RwLock::new(Stats::default())));
static CONFIG: LazyLock<Arc<RwLock<Config>>> = LazyLock::new(|| Arc::new(RwLock::new(Config::default())));
static MATRIX: LazyLock<Arc<RwLock<Vec<MachineInfo>>>> = LazyLock::new(|| Arc::new(RwLock::new(Vec::new())));

fn main() {
    configure();
    start_listening();
}

fn configure() {
    if let Ok(mut config) = CONFIG.write() {
        config.user_name = get_user_name();
        config.machine_id = rand::random();
        config.key = String::from("asdfasdfasdfasdf");
    }
}
