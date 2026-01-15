use crate::Hasher;
use std::net::TcpStream;

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
pub struct Stats {
    pub package_sent            : PackageMonitor,
    pub package_received        : PackageMonitor,
}

#[derive(Default, Debug)]
pub struct Config {
    pub user_name               : String,
    pub machine_name            : String, // INFO: also referred as hostname
    pub machine_id: u32,
    pub key: String,
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
    pub nil                     : u64,
    pub invalid: u64
}

#[derive(Debug)]
pub struct MachineInfo {
	pub name: String,
	pub id: u32,
	pub time: u128,
	pub hasher: Hasher,
	pub stream: TcpStream
}

pub enum SocketStatus {
	Connected,
	Disconnected
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
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

impl TryFrom<i32> for PackageType {
	type Error = &'static str;

	fn try_from(value: i32) -> Result<PackageType, <PackageType as TryFrom<i32>>::Error> {
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
	fn from(bytes: &[u8; 16]) -> MouseData {
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

#[derive(Default, Debug, Clone, Copy)]
pub struct MachineId {
	pub machine1: u32,
	pub machine2: u32,
	pub machine3: u32,
	pub machine4: u32,
}

#[derive(Clone, Copy)]
pub enum Message {
	DateTime(u64),
	KeyboardMsg(KeyboardData),
	MouseMsg(MouseData),
	ClipboardMsg(ClipboardPostAction),
	Machines(MachineId)
}

impl Message {
	pub fn from(bytes: [u8; 16], package_type: PackageType) -> Option<Message> {
		// TODO: have to check for byte vailidity, anyway for now just casting.
		match package_type {
			PackageType::Keyboard => {
				Some(Message::KeyboardMsg(KeyboardData::from(&bytes)))
			},
			PackageType::Mouse => {
				Some(Message::MouseMsg(MouseData::from(&bytes)))
			},
			PackageType::Handshake => {
				let mut machine = [0u8; 4];
				machine.copy_from_slice(bytes[..4].as_ref());
				let machine1 = u32::from_ne_bytes(machine);
				machine.copy_from_slice(bytes[4..8].as_ref());
				let machine2 = u32::from_ne_bytes(machine);
				machine.copy_from_slice(bytes[8..12].as_ref());
				let machine3 = u32::from_ne_bytes(machine);
				machine.copy_from_slice(bytes[12..].as_ref());
				let machine4 = u32::from_ne_bytes(machine);
				let message = Message::Machines(
					MachineId {
						machine1: machine1,
						machine2: machine2,
						machine3: machine3,
						machine4: machine4
					}
				);
				Some(message)
			}
			_ => None
		}
	}

	pub fn as_bytes(&self) -> [u8; 16] {
		match self {
			Message::DateTime(value) => {
				let mut bytes = [0u8; 16];
				bytes[..8].copy_from_slice(&value.to_ne_bytes());
				bytes
			},
			Message::KeyboardMsg(value) => {
				let mut bytes = [0u8; 16];
				bytes[..4].copy_from_slice(&value.vk.to_ne_bytes());
				bytes[4..8].copy_from_slice(&value.dw_flags.to_ne_bytes());
				bytes
			},
			Message::MouseMsg(value) => {
				let mut bytes = [0u8; 16];
				bytes[..4].copy_from_slice(&value.x.to_ne_bytes());
				bytes[4..8].copy_from_slice(&value.y.to_ne_bytes());
				bytes[8..12].copy_from_slice(&value.wheel_delta.to_ne_bytes());
				bytes[12..16].copy_from_slice(&value.dw_flags.to_ne_bytes());
				bytes
			},
			Message::ClipboardMsg(value) => {
				let mut bytes = [0u8; 16];
				bytes[..4].copy_from_slice(&(value.clone() as u32).to_ne_bytes());
				bytes
			},
			Message::Machines(value) => {
				let mut bytes = [0u8; 16];
				bytes[..4].copy_from_slice(&value.machine1.to_ne_bytes());
				bytes[4..8].copy_from_slice(&value.machine2.to_ne_bytes());
				bytes[8..12].copy_from_slice(&value.machine3.to_ne_bytes());
				bytes[12..16].copy_from_slice(&value.machine4.to_ne_bytes());
				bytes
			},
		}
	}
}

// MARK: payload
#[derive(Default, Clone)]
pub struct Data {
	// 4 bytes, layout: 0 - 3
	// byte0 = package type, byte1 = checksum, byte2 + byte3 = magic number
	pub package_type: Option<PackageType>,

	// 4 bytes, layout: 4 - 7
	pub id: i32,

	// 4 bytes, layout: 8 - 11
	pub src: u32,

	// 4 bytes, layout: 12 - 15
	pub dest: u32,

	// 16 byte union, layout: 16 - 31
	// originally the developer used explicit layout on struct
	// because csharp does not support unions
	pub message: Option<Message>,

	// layout: 32 - 63
	// originally machine name was splited into 4 u64.
	// but we can split it while convert to and from bytes
	pub machine_name: Option<String>
}

impl Data {
	pub fn from(bytes: &[u8; 32]) -> Self {

		let (package_type, rest) = bytes.split_first_chunk::<4>().unwrap();
		let (id, rest) = rest.split_first_chunk::<4>().unwrap();
		let (src, rest) = rest.split_first_chunk::<4>().unwrap();
		let (dest, rest) = rest.split_first_chunk::<4>().unwrap();
		let (message, rest) = rest.split_first_chunk::<16>().unwrap();

		let mut data = Self::default();

		let package_type = PackageType::try_from(i32::from_ne_bytes(package_type.to_owned()));
		if let Ok(package_type) = package_type {
			data.package_type = Some(package_type);
			data.message = Message::from(message.clone(), package_type);
		} else {
			data.message = None;
		}

		data.id = i32::from_ne_bytes(id.clone());
		data.src = u32::from_ne_bytes(src.clone());
		data.dest = u32::from_ne_bytes(dest.clone());

		data
	}

	pub fn update_machine_name(&mut self, machine_name: &[u8; 32]) {
		if let Ok(machine_name) = String::from_utf8(machine_name[32..].into()) {
			self.machine_name = Some(machine_name.trim().to_owned());
		}
	}

	pub fn as_bytes(&self) -> [u8; 64] {
		let mut bytes = [0u8; 64];

		if let Some(package_type) = self.package_type {
			bytes[..4].copy_from_slice(&(package_type as u32).to_ne_bytes().as_ref());
		}
		
		bytes[4..8].copy_from_slice(&self.id.to_ne_bytes().as_ref());
		bytes[8..12].copy_from_slice(&self.src.to_ne_bytes().as_ref());
		bytes[12..16].copy_from_slice(&self.dest.to_ne_bytes().as_ref());

		if let Some(message) = self.message {
			bytes[16..32].copy_from_slice(&message.as_bytes().as_ref());
		}

		let mut name = String::new();
		if let Some(machine_name) = self.machine_name.clone() {
			name.push_str(machine_name.as_str());
		}
		name.push_str(" ".repeat(32).as_ref());
		name.drain(32..);

		bytes[32..].copy_from_slice(name.as_bytes());

		bytes
	}

	pub fn is_big_package(&self) -> bool {
		let Some(package_type) = self.package_type else { return false; };

		match package_type {
			PackageType::Hello | PackageType::Awake | PackageType::Heartbeat | PackageType::HeartbeatEx | PackageType::Handshake | PackageType::HandshakeAck |
			PackageType::Clipboard | PackageType::ClipboardPush | PackageType::ClipboardAsk | PackageType::ClipboardImage | PackageType::ClipboardText | PackageType::ClipboardDataEnd => true,
			_ => (package_type as u32) & (PackageType::Matrix as u32) == (PackageType::Matrix as u32),
		}
	}

	// pub fn as_string(&self) -> String {
	// 	let mut output = String::new();

	// 	if let Some(package_type) = self.package_type {
	// 		match package_type {
				
	// 		}
	// 		bytes[..4].copy_from_slice(&(package_type as u32).to_ne_bytes().as_ref());
	// 	}
		
	// 	bytes[4..8].copy_from_slice(&self.id.to_ne_bytes().as_ref());
	// 	bytes[8..12].copy_from_slice(&self.src.to_ne_bytes().as_ref());
	// 	bytes[12..16].copy_from_slice(&self.dest.to_ne_bytes().as_ref());

	// 	if let Some(message) = self.message {
	// 		bytes[16..32].copy_from_slice(&message.as_bytes().as_ref());
	// 	}

	// 	let mut name = String::new();
	// 	if let Some(machine_name) = self.machine_name.clone() {
	// 		name.push_str(machine_name.as_str());
	// 	}
	// 	name.push_str(" ".repeat(32).as_ref());
	// 	name.drain(32..);

	// 	bytes[32..].copy_from_slice(name.as_bytes());

	// 	bytes
	// }
}
