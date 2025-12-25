mod darwin;

pub fn get_host_name() -> String {
    // TODO: implement for various os
    String::from("vpra")
}

pub fn get_user_name() -> (String, String) {
    darwin::get_user_name()
}

pub fn get_screen_size() -> darwin::NSRect {
    darwin::get_screen_size()
}
