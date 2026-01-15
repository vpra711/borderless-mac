// network related
use crate::structures::*;
use crate::hashing::Hasher;
use crate::syscalls::*;
use crate::logger::*;

use crate::CONFIG;

use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::{
    TcpListener,
    TcpStream,
    SocketAddr,
    Shutdown
};
use std::thread;
use std::thread_local;

pub fn start_listening() {

    let message_server = SocketAddr::from(([0, 0, 0, 0], 15101));
    let clipboard_server = SocketAddr::from(([0, 0, 0, 0], 15100));
    let message_listener = TcpListener::bind(&message_server);
    let clipboard_listener = TcpListener::bind(&clipboard_server);

    let message_server_thread = thread::spawn(|| {
        let Ok(message_streams) = message_listener else {
            eprintln!("error: unable to bind to message port: 15101");
            return
        };
        println!("info: listening on port: 15101");
        for streams in message_streams.incoming() {
            let Ok(mut stream) = streams else {
                eprintln!("error: new tcp incomming message connction rejected");
                return
            };
            println!("info: new incomming tcp message connection accepted and listening...");
            stream.set_nodelay(true);
            if let Some(mut hasher) = create_hasher(&mut stream) {
                handle_message_stream(&mut stream, &mut hasher);
            }
        }
    });

    let clipboard_server_thread = thread::spawn(|| {
        let Ok(clipboard_streams) = clipboard_listener else {
            eprintln!("error: unable to bind to clipboard port: 15100");
            return
        };
        println!("info: listening on port: 15100");
        for streams in clipboard_streams.incoming() {
            let Ok(mut stream) = streams else {
                eprintln!("error: new tcp incomming clipboard connction rejected");
                return
            };
            stream.set_nodelay(true);
            println!("info: new incomming tcp clipboard connection accepted and listening...");
            println!("info: clipboard functionality not yet implemented...");
        }
    });

    message_server_thread.join();
    clipboard_server_thread.join();
}

pub fn create_hasher(stream: &mut TcpStream) -> Option<Hasher> {
    let mut received_package_count = 0;
    let Ok(config) = CONFIG.read() else {
        eprintln!("error: unable to lock n read the CONFIG - requires key to create new hasher");
        return None
    };
    let mut hasher = Hasher::from(config.key.clone());
    // to spend initial vector which is 16 bytes.
    // so that actual data will be encyrpted and decrypted with previous iv.
    let mut ignore_iv_bytes = [0u8; 16];
    stream.read_exact(&mut ignore_iv_bytes);
    hasher.decrypt_data(&mut ignore_iv_bytes);
    rand::fill(&mut ignore_iv_bytes);
    hasher.encrypt_data(&mut ignore_iv_bytes);
    stream.write_all(&ignore_iv_bytes);
    Some(hasher)
}

pub fn handle_message_stream(stream: &mut TcpStream, hasher: &mut Hasher) {
    let mut received_package_count = 0;
    let mut error_package_count = 0;
    let mut received_handshake_package = MachineId::default();
    loop {
        let mut data = tcp_read(stream, hasher);
        let Some(package_type) = data.package_type else {
            eprintln!("error: package has no type - unable to interpret the data");
            data.package_type = Some(PackageType::Invalid);
            for _ in 0..10 {
                tcp_write(stream, hasher, &mut data);
            }
            stream.shutdown(Shutdown::Both);
            return
        };

        if package_type == PackageType::Error {
            error_package_count += 1;
            if received_package_count > 0 {
                println!("info: received an invalid package. check the security keys");
            }
            if error_package_count > 5 {
                stream.shutdown(Shutdown::Both);
                println!("info: closing socket due to high volume of error packages");
                return
            }
            continue
        }
        error_package_count = 0;

        if package_type == PackageType::Handshake {
            if let Some(machines) = data.message {
                if let Message::Machines(values) = machines {
                    received_handshake_package = values;
                }
            }
            perform_handshake(stream, hasher, &mut data);
            continue
        }

        // pre-checks
        if received_package_count > -1 {
            received_package_count += 1;
            if received_package_count > 9 {
                eprintln!("error: closing socket due to high volume of invalid packages");
                data.package_type = Some(PackageType::Invalid);
                for _ in 0..10 {
                    tcp_write(stream, hasher, &mut data);
                }
                stream.shutdown(Shutdown::Both);
                return
            }

            match package_type {
                PackageType::HandshakeAck => {
                    let Some(machines) = data.message else {
                        println!("info: error invalid handshake data received");
                        break
                    };
                    let Message::Machines(values) = machines else {
                        println!("info: error invalid handshake data received");
                        break
                    };
                    if !values.machine1 == received_handshake_package.machine1 &&
                        !values.machine2 == received_handshake_package.machine2 &&
                        !values.machine3 == received_handshake_package.machine3 &&
                        !values.machine4 == received_handshake_package.machine4 {
                        received_package_count = -1;
                    } else {
                        println!("info: error invalid handshake data received");
                        break
                    }
                },
                PackageType::Mouse | PackageType::Heartbeat | PackageType::HeartbeatEx => {
                    if received_package_count > 5 {
                        received_package_count -=1;
                    }
                },
                _ => {
                    if received_package_count > 5 {
                        // TODO: update the machine socket status to invalid key
                        
                    } else {
                        println!("info: unexpected package");
                    }
                }
            }
            continue
        }

        // main decision making
        match package_type {
            PackageType::Handshake => {
                eprintln!("error: handshake at this point should not be possible");
            },
            PackageType::HandshakeAck => {
                println!("info: skipping rest of handshake ack");
            },
            PackageType::Invalid => {
                println!("info: invalid package received");
                if let Ok(mut stats) = crate::STATS.write() {
                    stats.package_received.invalid += 1;
                }
            },
            PackageType::Error => {},
            PackageType::Hi => { println!("packageType = Hi ")},
            PackageType::Hello => { println!("packageType = Hello ")},
            PackageType::ByeBye => { println!("packageType = ByeBye ")},
            PackageType::Heartbeat => { println!("packageType = Heartbeat ")},
            PackageType::Awake => { println!("packageType = Awake ")},
            PackageType::HideMouse => { println!("packageType = HideMouse ")},
            PackageType::HeartbeatEx => { println!("packageType = HeartbeatEx ")},
            PackageType::HeartbeatExL2 => { println!("packageType = HeartbeatExL2 ")},
            PackageType::HeartbeatExL3 => { println!("packageType = HeartbeatExL3 ")},
            PackageType::Clipboard => { println!("packageType = Clipboard ")},
            PackageType::ClipboardDragDrop => { println!("packageType = ClipboardDragDrop ")},
            PackageType::ClipboardDragDropEnd => { println!("packageType = ClipboardDragDropEnd ")},
            PackageType::ExplorerDragDrop => { println!("packageType = ExplorerDragDrop ")},
            PackageType::ClipboardCapture => { println!("packageType = ClipboardCapture ")},
            PackageType::CaptureScreenCommand => { println!("packageType = CaptureScreenCommand ")},
            PackageType::ClipboardDragDropOperation => { println!("packageType = ClipboardDragDropOperation ")},
            PackageType::ClipboardDataEnd => { println!("packageType = ClipboardDataEnd ")},
            PackageType::MachineSwitched => { println!("packageType = MachineSwitched ")},
            PackageType::ClipboardAsk => { println!("packageType = ClipboardAsk ")},
            PackageType::ClipboardPush => { println!("packageType = ClipboardPush ")},
            PackageType::NextMachine => { println!("packageType = NextMachine ")},
            PackageType::Keyboard => { println!("packageType = Keyboard ")},
            PackageType::Mouse => { println!("packageType = Mouse ")},
            PackageType::ClipboardText => { println!("packageType = ClipboardText ")},
            PackageType::ClipboardImage => { println!("packageType = ClipboardImage ")},
            PackageType::Matrix => { println!("packageType = Matrix ")},
        }
    }
}

pub fn tcp_read(stream: &mut TcpStream, hasher: &mut Hasher) -> Data {
    let mut message = [0u8; 32];
    let mut machine_name = [0u8; 32];
    let mut data = Data::from(&message);

    if let Err(error) = stream.read_exact(&mut message) {
        eprintln!("error: error reading the first 32 bytes");
        eprintln!("{}", error);
        data.package_type = Some(PackageType::Error);
        return data;
    }
    hasher.decrypt_data(&mut message);
    hasher.verify_and_update_package(&mut message);
    data = Data::from(&message);
    if data.is_big_package() {
        if let Err(error) = stream.read_exact(&mut machine_name) {
            eprintln!("error: big package received - error reading the remaining bytes");
            eprintln!("{}", error);
            data.package_type = Some(PackageType::Error);
            return data;
        }
        hasher.decrypt_data(&mut machine_name);
        data.update_machine_name(&machine_name);
    }
    return data;
}

pub fn tcp_write(stream: &mut TcpStream, hasher: &mut Hasher, data: &mut Data) {
    // preprocess_data(data);
    if data.src == 0 && let Ok(config) = CONFIG.read() {
        data.src = config.machine_id;
    }

    let mut bytes = data.as_bytes();
    hasher.sign_package(bytes.split_first_chunk_mut::<32>().unwrap().0);
    hasher.encrypt_data(&mut bytes);
    if let Ok(_) = stream.write_all(bytes.as_ref()) {
        stream.flush();
    } else {
        eprintln!("error: tcp write failed");
    }
}

pub fn preprocess_data(data: &Data) {
    let Some(package_type) = data.package_type else { return };
    let Ok(mut config) = CONFIG.write() else { return };
}

pub fn perform_handshake(stream: &mut TcpStream, hasher: &mut Hasher, data: &mut Data) {
    println!("info: handshake");
    data.package_type = Some(PackageType::HandshakeAck);
    data.src = 0;
    if let Ok(mut config) = CONFIG.read() {
        data.machine_name = Some(config.machine_name.clone());
    }
    let Some(machines) = data.message.clone() else {
        eprintln!("message in data is None");
        return
    };
    let Message::Machines(value) = machines else {
        eprintln!("message is not relate to machine ids");
        return
    };
    let inverted_machine_id = Message::Machines(
        MachineId {
            machine1: !value.machine1,
            machine2: !value.machine2,
            machine3: !value.machine3,
            machine4: !value.machine4
        }
    );
    data.message = Some(inverted_machine_id);
    tcp_write(stream, hasher, data);
}
