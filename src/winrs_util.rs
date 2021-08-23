
mod winrs {
	pub use bindings::{
		Windows::Win32::Foundation::{
			HWND, PWSTR, LPARAM, WPARAM, HANDLE, 
		},
		Windows::Win32::UI::WindowsAndMessaging::{
			GetMessageW, PeekMessageW, SendMessageW,
	
			// Apis
			WM_GETTEXTLENGTH, WM_GETTEXT, 

		},
		Windows::Win32::UI::Shell::{
			StrCpyW, 
		},
		Windows::Win32::System::Memory::{
			GlobalAlloc, GlobalLock, GlobalUnlock, GlobalFree, 
			GHND, 
		},
		Windows::Win32::System::DataExchange::{
			OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard, 
		},
		Windows::Win32::System::SystemServices::{
			CF_UNICODETEXT,
		}
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

pub fn wparam2command_button(wparam: winrs::WPARAM) -> (usize, usize) {
	let child_id = wparam.0 & 0x00000000FFFFFFFF as usize;
	let notify = (wparam.0 & 0xFFFFFFFF00000000) >> 32 as usize;
	(notify, child_id)
}

pub fn set_clipboard(buff: *mut u16, size: usize) {
	unsafe {
		// HGLOBALの定義がない
		let h_data = winrs::GlobalAlloc(winrs::GHND, (size+1)*2);
		let p_data = winrs::GlobalLock(h_data);
		if p_data.is_null() {
			println!("failed GlobalLock.");
			winrs::GlobalUnlock(h_data);
			return;
		}
		{
			let ptr_dst = p_data as *mut u16;
			let ptr_src = buff as *const u16;
			let mut data: u16;
			for count in 0..size {
				let idx: isize = count as isize;
				data = ptr_src.offset(idx + 0).read();
				ptr_dst.offset(idx + 0).write_unaligned(data);
			}
		};
		//let str_dst = winrs::PWSTR(p_data as *mut u16);
		//let str_src = winrs::PWSTR(buff as *mut u16);
		//winrs::StrCpyW(str_dst, str_src);
		winrs::GlobalUnlock(h_data);
		// 
		let op = winrs::OpenClipboard(winrs::HWND(0));
		if op.as_bool() {
			winrs::EmptyClipboard();
			let set_result = winrs::SetClipboardData(winrs::CF_UNICODETEXT.0, winrs::HANDLE(h_data));
			if set_result.is_null() {
				winrs::GlobalFree(h_data);
			}
			winrs::CloseClipboard();
		}
	}
}

pub fn edit_get_text(hwnd: winrs::HWND, buff: *mut u16, size: usize) -> usize {
	unsafe {
		// バッファにEditControlの内容をコピー
		// 返り値は\0を含まないコピー文字数
		let whole_len = winrs::SendMessageW(
			hwnd, winrs::WM_GETTEXT,
			winrs::WPARAM(size),
			winrs::LPARAM(buff as isize)
		);

		whole_len.0 as usize
	}
}
