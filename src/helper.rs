// helper.rs
 
fn setup_machine_name_and_id(config: &mut Config) {
    config.machine_name = get_host_name();
    initialize_machine_pool(config);
}


fn initialize_machine_pool(config: &mut Config) {
    // TODO: for prototyping i hardcoreded the string
    // change it to store in database or some sort of file
    // sqlite would be good.
    let mut milli_seconds = SystemTime::now().elapsed().unwrap().as_millis();

    // simulate fetching from database/file.
    let integer_id: u32 = 0;
    
 
    for machine in &mut config.machines {
        machine.name = machine.name.trim().to_string();
        // i can store u64 in database
        machine.id = Id::try_from(integer_id).unwrap();
        if machine.id == Id::None {
            machine.time = milli_seconds - HEARTBEAT_TIMEOUT as u128;
        } else {
            machine.time = milli_seconds;
        }
    }
}

fn initialize_machines(machines: [MachineInfo; 4]) {
    for machine in machines {
        learn_machine(machine, config);
        try_update_machine_and_id();
    }
}

fn learn_machine(machine_name: String, config: &mut Config) -> bool {
    if config.machines.len() > MAX_MACHINE {
        // machine slots are full
        return false;
    }

    if config.machines.iter().any(|machine| machine.name == machine_name) {
        // machine already added
        return false;
    }

    config.machines.push(
        MachineInfo {
            name: machine_name,
            id: Id::None,
            time: SystemTime::now().elapsed().unwrap().as_millis()
        }
    );

    return true;
}
