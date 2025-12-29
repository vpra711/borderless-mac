use crate::constants::*;

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
    pub machine_name            : String, // INFO: also referred as hostname
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

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum PackageType {
    Invalid = 0xFF,
    Error = 0xFE,
    Hi = 2,
    Hello = 3,
    ByeBye = 4,
    Heartbeat = 20,
    Awake = 21,
    HideMouse = 50,
    HeartbeatEx = 51,
    HeartbeatExL2 = 52,
    HeartbeatExL3 = 53,
    Clipboard = 69,
    ClipboardDragDrop = 70,
    ClipboardDragDropEnd = 71,
    ExplorerDragDrop = 72,
    ClipboardCapture = 73,
    CaptureScreenCommand = 74,
    ClipboardDragDropOperation = 75,
    ClipboardDataEnd = 76,
    MachineSwitched = 77,
    ClipboardAsk = 78,
    ClipboardPush = 79,
    NextMachine = 121,
    Keyboard = 122,
    Mouse = 123,
    ClipboardText = 124,
    ClipboardImage = 125,
    Handshake = 126,
    HandshakeAck = 127,
    Matrix = 128,
    // matrixSwapFlag = 2,
    // matrixTwoRowFlag = 4,
}

impl TryFrom<u32> for PackageType {
	type Error = &'static str;

	fn try_from(value: u32) -> Result<PackageType, <PackageType as TryFrom<u32>>::Error> {
		match value {
		    0xFF => Ok(PackageType::Invalid), 
		    0xFE => Ok(PackageType::Error), 
		    2    => Ok(PackageType::Hi), 
		    3    => Ok(PackageType::Hello), 
		    4    => Ok(PackageType::ByeBye), 
		    20   => Ok(PackageType::Heartbeat), 
		    21   => Ok(PackageType::Awake), 
		    50   => Ok(PackageType::HideMouse), 
		    51   => Ok(PackageType::HeartbeatEx), 
		    52   => Ok(PackageType::HeartbeatExL2), 
		    53   => Ok(PackageType::HeartbeatExL3), 
		    69   => Ok(PackageType::Clipboard), 
		    70   => Ok(PackageType::ClipboardDragDrop), 
		    71   => Ok(PackageType::ClipboardDragDropEnd), 
		    72   => Ok(PackageType::ExplorerDragDrop), 
		    73   => Ok(PackageType::ClipboardCapture), 
		    74   => Ok(PackageType::CaptureScreenCommand), 
		    75   => Ok(PackageType::ClipboardDragDropOperation), 
		    76   => Ok(PackageType::ClipboardDataEnd), 
		    77   => Ok(PackageType::MachineSwitched), 
		    78   => Ok(PackageType::ClipboardAsk), 
		    79   => Ok(PackageType::ClipboardPush), 
		    121  => Ok(PackageType::NextMachine), 
		    122  => Ok(PackageType::Keyboard), 
		    123  => Ok(PackageType::Mouse), 
		    124  => Ok(PackageType::ClipboardText), 
		    125  => Ok(PackageType::ClipboardImage), 
		    126  => Ok(PackageType::Handshake), 
		    127  => Ok(PackageType::HandshakeAck), 
		    128  => Ok(PackageType::Matrix), 
		    _    => Err("unknown value")
	    }
 	}
}

impl Default for PackageType {
	fn default() -> PackageType {
		PackageType::Hi
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Id {
	None = 0,
	All = 255
}

impl TryFrom<u32> for Id {
	type Error = &'static str;

	fn try_from(value: u32) -> Result<Self, <Id as TryFrom<u32>>::Error> {
		match value {
			0   => Ok(Id::None),
			255 => Ok(Id::All),
			_   => Err("unknown value")
		}
	}
}

impl Default for Id {
	fn default() -> Id {
		Id::None
	}
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardData {
	pub vk: i32,
	pub dw_flags: i32
}

impl KeyboardData {
	fn from(bytes: &[u8; 16]) -> KeyboardData {
		KeyboardData {
			vk: i32::from_ne_bytes(bytes[..4].try_into().unwrap()),
			dw_flags: i32::from_ne_bytes(bytes[4..8].try_into().unwrap())
		}
	}

	fn as_bytes(&self) -> [u8; 16] {
		let mut bytes = [0u8; 16];
		bytes[..4].copy_from_slice(&self.vk.to_ne_bytes());
		bytes[4..8].copy_from_slice(&self.dw_flags.to_ne_bytes());
		bytes
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MouseData {
    x: i32,
    y: i32,
    wheel_delta: i32,
    dw_flags: i32
}

impl MouseData {
	fn from(bytes: [u8; 16]) -> MouseData {
		MouseData {
			x: i32::from_ne_bytes(bytes[..4].try_into().unwrap()),
			y: i32::from_ne_bytes(bytes[4..8].try_into().unwrap()),
			wheel_delta: i32::from_ne_bytes(bytes[8..12].try_into().unwrap()),
			dw_flags: i32::from_ne_bytes(bytes[12..16].try_into().unwrap())
		}
	}

	fn as_bytes(&self) -> [u8; 16] {
		let mut bytes = [0u8; 16];
		bytes[..4].copy_from_slice(&self.x.to_ne_bytes());
		bytes[4..8].copy_from_slice(&self.y.to_ne_bytes());
		bytes[8..12].copy_from_slice(&self.wheel_delta.to_ne_bytes());
		bytes[12..16].copy_from_slice(&self.dw_flags.to_ne_bytes());
		bytes
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum ClipboardPostAction {
	Other = 0,
	Desktop = 1,
	MsPaint = 2
}

impl TryFrom<u32> for ClipboardPostAction {
	type Error = &'static str;

	fn try_from(value: u32) -> Result<ClipboardPostAction, <ClipboardPostAction as TryFrom<u32>>::Error> {
		match value {
			0 => Ok(ClipboardPostAction::Other),
			1 => Ok(ClipboardPostAction::Desktop),
			2 => Ok(ClipboardPostAction::MsPaint),
			_ => Err("unknown value")
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MachineId {
	pub machine1: Id,
	pub machine2: Id,
	pub machine3: Id,
	pub machine4: Id,
}

impl Default for Message {
	fn default() -> Message {
		Message {
			date_time: 9999999999999
		}
	}
}

#[derive(Clone, Copy)]
pub union Message {
	pub date_time: u64,
	pub keyboard_data: KeyboardData,
	pub mouse_data: MouseData,
	pub clipboard_action: ClipboardPostAction
}

impl Message {
	fn from(bytes: &[u8; 16]) -> Message {
		unsafe {
			let ptr: *const u8 = bytes.first().unwrap();
			let message: *const Message = ptr as *const Message;
			*message
		}
	}

	fn as_bytes(&self) -> [u8; 16] {
		let mut bytes = [0u8; 16];
		unsafe {
			let ptr: *const Message = self;
			let raw_bytes = std::slice::from_raw_parts(ptr as *const u8, std::mem::size_of::<Message>());
			bytes.copy_from_slice(raw_bytes);
			bytes
		}
	}

	fn as_string(&self, package_type: PackageType) -> String {
		// close eyes and cast
		match package_type {
			PackageType::Keyboard => {
				unsafe {
					let mut value = String::new();
					value.push_str("vk: ");
					value.push_str(&self.keyboard_data.vk.to_string());
					value.push_str(", dw_flags: ");
					value.push_str(&self.keyboard_data.dw_flags.to_string());
					value
				}
			},
			PackageType::Mouse | PackageType::NextMachine => {
				unsafe {
					let mut value = String::new();
					value.push_str("x: ");
					value.push_str(&self.mouse_data.x.to_string());
					value.push_str(", y: ");
					value.push_str(&self.mouse_data.y.to_string());
					value.push_str(", wheel_delta: ");
					value.push_str(&self.mouse_data.wheel_delta.to_string());
					value.push_str(", dw_flags: ");
					value.push_str(&self.mouse_data.dw_flags.to_string());
					value
				}
			},
			_ => String::from("message: none")
		}
	}
}

// MARK: payload
#[derive(Default, Clone)]
pub struct Data {
	// 4 bytes, layout: 0 - 3
	pub package_type: PackageType,

	// 4 bytes, layout: 4 - 7
	pub id: u32,

	// 4 bytes, layout: 8 - 11
	pub src: Id,

	// 4 bytes, layout: 12 - 15
	pub dest: Id,

	// 16 byte union, layout: 16 - 31
	// originally the developer used explicit layout on struct
	// because csharp does not support unions
	// TODO: convert to enum
	pub message: Message,

	// originally 4 * i64, the machine name was splited into 4 parts
	// since sending the bytes in steam, we dont have to split it
	// in the struct itself. see serializing.
	// assumed max of 32 character, > 32 will be stripped
	// layout: 32 - 63
	pub machine_name: String
}

impl Data {
	pub fn from(bytes: &[u8; 64]) -> Result<Self, &str> {

		let half         = bytes.as_chunks::<32>();
		let package_info = half.0[0];
		let machine_name = String::from_utf8(half.0[1].to_vec()).unwrap().trim().to_string();

		let message      = package_info.as_chunks::<16>();
		let meta_data    = message.0[0];
		let message      = message.0[1];

		let meta_data    = meta_data.as_chunks::<4>();
		let package_type = u32::from_ne_bytes(meta_data.0[0]);
		let id           = u32::from_ne_bytes(meta_data.0[1]);
		let src          = u32::from_ne_bytes(meta_data.0[2]);
		let dest         = u32::from_ne_bytes(meta_data.0[3]);

		Ok(
			Self {
				package_type: PackageType::try_from(package_type).unwrap(),
				id: id,
				src: Id::try_from(src).unwrap(),
				dest: Id::try_from(dest).unwrap(),
				message: Message::from(&message),
				machine_name: machine_name
			}
		)
	}

	pub fn as_bytes(&self) -> [u8; 64] {
		let mut bytes = [0u8; PAYLOAD_LENGTH];

		bytes[..4].copy_from_slice(&(self.package_type as u32).to_ne_bytes());
		bytes[4..8].copy_from_slice(&self.id.to_ne_bytes());
		bytes[8..12].copy_from_slice(&(self.src as u32).to_ne_bytes());
		bytes[12..16].copy_from_slice(&(self.dest as u32).to_ne_bytes());
		bytes[16..32].copy_from_slice(&self.message.as_bytes());

		bytes[32..].fill(b' ');
		let machine_name = self.machine_name.as_bytes().get(..32).unwrap_or(self.machine_name.as_bytes());
		bytes[32..(32 + machine_name.len())].copy_from_slice(machine_name);
		bytes
	}

	pub fn print(&self) {
		println!(
			"type        : {:0?}
			id           : {:1?}
			src          : {:2?}
			dest         : {:3?}
			message      : {:4?}
			machine_name : {:5?}",
			self.package_type,
			self.id,
			self.src,
			self.dest,
			self.message.as_string(self.package_type),
			self.machine_name
		)
	}
}
