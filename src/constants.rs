pub const INITINAL_VECTOR: u64            = u64::MAX;
pub const MAX_MACHINE: usize                = 4;
pub const MAX_SOCKET: usize = MAX_MACHINE * 2;
pub const HEARTBEAT_TIMEOUT: u32         = 1500000;
pub const SKIP_PIXELS: i8                = 1;
pub const JUMP_PIXELS: i8                = 2;
pub const TCP_PORT_CLIPBOARD: u16 = 15100;
pub const TCP_PORT_MESSAGE: u16   = TCP_PORT_CLIPBOARD + 1;
pub const PACKET_SIZE: usize = 32;
pub const EXTENDED_PACKET_SIZE: u32 = 64;
pub const POSSIBLE_PASSWD_CHARS: &str =
"abcdefghjkmnpqrstuvxyz\
ABCDEFGHJKMNPQRSTUVXYZ\
123456789\
~!@#$%^*()_-+=:;<,>.?/\\|[]";
pub const PASSWD_LENGTH: usize = 16;
pub const PAYLOAD_LENGTH: usize = 64;
