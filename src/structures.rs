#[derive(Default, Debug)]
pub struct MouseLocation {
	pub x                       : i32,
	pub y                       : i32,
	pub count                   : i32 // what count for?
}

// when connecting multiple computers entire screen size will change
#[derive(Default, Debug)]
pub struct ScreenSize {
	pub left                    : i32,
	pub top                     : i32,
	pub right                   : i32,
	pub bottom                  : i32
}

#[derive(Default, Debug)]
pub struct Config {
	pub my_key                  : String,
	pub key_generated           : bool,
	pub key_corrupted           : bool,
    pub package_sent            : PackageMonitor,
    pub package_received        : PackageMonitor,
    pub package_id              : u32,
    pub running_as_system       : bool,
    pub running_w_admin_right   : bool,
    pub user_name               : String,
    pub machine_name            : String // INFO: also referred as hostname
}

#[derive(Default, Debug)]
pub struct PackageMonitor {
    pub keyboard                : u64,
    pub mouse                   : u64,
    pub heart_beat              : u64,
    pub bye_bye                 : u64,
    pub hello                   : u64,
    pub matrix                  : u64,
    pub clipboard               : u64,
    pub clipboard_text          : u64,
    pub clipboard_image         : u64,
    pub clipboard_drag_drop     : u64,
    pub clipboard_drag_drop_end : u64,
    pub clipboard_ask           : u64,
    pub explorer_drag_drop      : u64,
    pub nil                     : u64
}
