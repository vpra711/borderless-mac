pub const INITINAL_VALUE: u64            = u64::MAX;
pub const MAX_MACHINE: i8                = 4;
pub const MAX_SOCKET: i8                 = MAX_MACHINE * 2;
pub const HEARTBEAT_TIMEOUT: i32         = 1500000;
pub const SKIP_PIXELS: i8                = 1;
pub const JUMP_PIXELS: i8                = 2;
pub const TCP_PORT_CLIPBOARD: i16 = 15100;
pub const TCP_PORT_MESSAGE: i16   = TCP_PORT_CLIPBOARD + 1;
pub const PACKET_SIZE: u32 = 32 * 10000;
pub const EXTENDED_PACKET_SIZE: u32 = 64 * 10000;
pub const POSSIBLE_PASSWD_CHARS: &str =
"abcdefghjkmnpqrstuvxyz\
ABCDEFGHJKMNPQRSTUVXYZ\
123456789\
~!@#$%^*()_-+=:;<,>.?/\\|[]";
pub const PASSWD_LENGTH: usize = 16;
