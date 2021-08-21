mod winrs {
	pub use bindings::{
		Windows::Win32::Foundation::{
			PWSTR, LPARAM,
		},
	};
}

pub trait WSTRTrait<T> {
	fn new(u8str: T) -> WSTR;
}
pub struct WSTR {
	buff: Vec<u16>,
}
impl WSTRTrait<&String> for WSTR {
	fn new(u8str: &String) -> WSTR {
		WSTR{
			buff: u8str.encode_utf16().collect(),
		}
	}
}
impl WSTRTrait<&str> for WSTR {
	fn new(u8str: &str) -> WSTR {
		WSTR{
			buff: u8str.encode_utf16().collect(),
		}
	}
}
impl WSTR {
	pub fn as_pwstr(&self) -> winrs::PWSTR {
		winrs::PWSTR(self.buff.as_ptr() as *mut u16)
	}
}

pub struct WndCoord {
	x: i32,
	y: i32,
	w: i32,
	h: i32,
	child: Vec<Self>,
}
impl WndCoord {
	const PADDING: i32 = 5;

	pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
		WndCoord{
			x,
			y,
			w: width,
			h: height,
			child: vec![]
		}
	}

	pub fn update_root(&mut self, x: i32, y: i32, width: i32, height: i32) {
		self.x = x;
		self.y = y;
		self.w = width;
		self.h = height;
	}

	pub fn add_child(&mut self, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
		// PADDINGチェック
		if x < Self::PADDING {
			x = Self::PADDING;
		}
		if y < Self::PADDING {
			y = Self::PADDING;
		}
		if (x + width) > (self.w - Self::PADDING) {
			width = self.w - Self::PADDING * 2;
		}
		if (y + height) > (self.h - Self::PADDING) {
			height = (self.h - Self::PADDING) - y;
		}
		// 登録
		let child :WndCoord = WndCoord::new(x,y,width,height);
		self.child.push(child);
	}

	pub fn update_child(&mut self, idx: usize) {
		let child: Option<&mut WndCoord> = self.child.get_mut(idx);
		match child {
			Some(_child) => {
				// PADDINGチェック
				// if x < Self::PADDING {
				// 	x = Self::PADDING;
				// }
				// if y < Self::PADDING {
				// 	y = Self::PADDING;
				// }
				_child.w = self.w - Self::PADDING * 2;
				_child.h = (self.h - Self::PADDING) - _child.y;
			}
			None => panic!("invalid index!")
		}
	}

	pub fn get_child(&self, idx: usize) -> &WndCoord {
		let child = self.child.get(idx);
		match child {
			Some(_child) => _child,
			None => panic!("invalid index!")
		}
	}

	pub fn x(&self) -> i32 {
		self.x
	}
	pub fn y(&self) -> i32 {
		self.y
	}
	pub fn width(&self) -> i32 {
		self.w
	}
	pub fn height(&self) -> i32 {
		self.h
	}
}

pub fn lparam2size(lparam: winrs::LPARAM) {
	let width = lparam.0 & 0x0000FFFF;
}
