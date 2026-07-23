mod darwin;

pub fn get_host_name() -> String {
    darwin::get_process_info().2
}

pub fn get_user_name() -> String {
    darwin::get_process_info().0
}

pub fn get_full_user_name() -> String {
    darwin::get_process_info().1
}

pub fn get_screen_size() -> darwin::NSRect {
    darwin::get_screen_size()
}

pub fn get_remote_host_name() -> String {
    String::new()
}
