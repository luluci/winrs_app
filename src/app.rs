
use std::ffi::c_void;

use bindings::{
	Windows::Win32::Foundation::{
		HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR,
		RECT,
	},
	Windows::Win32::UI::WindowsAndMessaging::{
		GetMessageW, PeekMessageW, SendMessageW,
		CreateWindowExW, DefWindowProcW, DispatchMessageW, PostQuitMessage,
		RegisterClassW, MessageBoxW, TranslateMessage, SendDlgItemMessageW, 
		GetClientRect, MoveWindow, 
		MSG, WNDCLASSW, HMENU, CREATESTRUCTW, CW_USEDEFAULT,
		// Apis
		WM_DESTROY, WM_PAINT, WM_CREATE, WM_QUIT, WM_SIZE, WM_GETTEXTLENGTH, WM_COMMAND, 
		WINDOW_STYLE,
		WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CHILDWINDOW, WS_CHILD, WS_BORDER, WS_VSCROLL, WM_SETFONT, WM_DROPFILES, 
		WS_CLIPCHILDREN, WS_CLIPSIBLINGS, 
		ES_AUTOHSCROLL, ES_MULTILINE, 
		WINDOW_EX_STYLE,
		WS_EX_ACCEPTFILES,
		// WNDCLASS_STYLES,
		CS_HREDRAW, CS_VREDRAW,
		// MESSAGEBOX_STYLE,
		MB_OK,
		LoadCursorW, IDC_ARROW,
		PEEK_MESSAGE_REMOVE_TYPE,
		PM_REMOVE, 
		SetWindowLongPtrW, CallWindowProcW,
		GWLP_WNDPROC, 
		WNDPROC, 
	},
	Windows::Win32::UI::Controls::{
		InitCommonControlsEx,
		INITCOMMONCONTROLSEX, INITCOMMONCONTROLSEX_ICC, 
		EM_SETSEL, EM_REPLACESEL, 
	},
	Windows::Win32::UI::Shell::{
		SetWindowSubclass, DefSubclassProc, 
		HDROP, DragQueryFileW, DragFinish, 
	},
	Windows::Win32::System::LibraryLoader::{
		GetModuleHandleW
	},
	Windows::Win32::Graphics::Gdi::{
		ValidateRect, GetStockObject, UpdateWindow,
		HBRUSH, GET_STOCK_OBJECT_FLAGS, DeleteObject, 
		DKGRAY_BRUSH,
		HGDIOBJ,
		HFONT, CreateFontW, FW_NORMAL, SHIFTJIS_CHARSET, OUT_TT_PRECIS, CLIP_DEFAULT_PRECIS,  
		DEFAULT_QUALITY, FF_MODERN, 
	},
};
use once_cell::sync::OnceCell;

use crate::{app::resource::UTF16_CRLF, winrs_util};
use crate::winrs_util::WSTRTrait;

// static resource
mod resource {
	use once_cell::sync::OnceCell;
	use crate::winrs_util;
	use crate::winrs_util::WSTRTrait;

	// root window info
	pub static CLASS_NAME: OnceCell<winrs_util::WSTR> = OnceCell::new();
	pub static WND_TITLE: OnceCell<winrs_util::WSTR> = OnceCell::new();
	// EDIT window info
	pub static EDIT_CLASS_NAME: OnceCell<winrs_util::WSTR> = OnceCell::new();
	pub static EDIT_DEF_STR: OnceCell<winrs_util::WSTR> = OnceCell::new();
	// BUTTON window info
	pub static BUTTON_CLASS_NAME: OnceCell<winrs_util::WSTR> = OnceCell::new();
	pub static BUTTON_DISP: OnceCell<winrs_util::WSTR> = OnceCell::new();
	// 
	pub static UTF16_CR: OnceCell<u16> = OnceCell::new();
	pub static UTF16_LF: OnceCell<u16> = OnceCell::new();
	pub static UTF16_CRLF: OnceCell<winrs_util::WSTR> = OnceCell::new();
	pub static UTF16_NULL: OnceCell<u16> = OnceCell::new();

	pub fn init() {
		CLASS_NAME.set(winrs_util::WSTR::new("MyWindowClass\0"));
		WND_TITLE.set(winrs_util::WSTR::new("Win32 App\0"));
		EDIT_CLASS_NAME.set(winrs_util::WSTR::new("EDIT\0"));
		EDIT_DEF_STR.set(winrs_util::WSTR::new("初期テキスト\0"));
		BUTTON_CLASS_NAME.set(winrs_util::WSTR::new("BUTTON\0"));
		BUTTON_DISP.set(winrs_util::WSTR::new("クリップボードにコピー\0"));
		UTF16_CR.set("\r".encode_utf16().next().unwrap());
		UTF16_LF.set("\n".encode_utf16().next().unwrap());
		UTF16_CRLF.set(winrs_util::WSTR::new("\r\n\0"));
		UTF16_NULL.set("\0".encode_utf16().next().unwrap());
	}
}
// dynamic resource
// use std::cell::RefCell;
// thread_local!(static INSTANCE2: RefCell<App> = RefCell::new(App::new()));
// use std::sync::Mutex;
// use once_cell::sync::Lazy;
// static INSTANCE: Lazy<Mutex<App>> = Lazy::new(||{
// 	Mutex::new(App::new())
// });
static mut INSTANCE: OnceCell<App> = OnceCell::new();

pub fn run() {
	// リソース初期化
	resource::init();
	//
	unsafe {
		INSTANCE.set(App::new());
		INSTANCE.get_mut().unwrap().init();
	}
	// {
	// 	// アプリ初期設定
	// 	// 設定のみ実施してMutexを解放する
	// 	let mut app = INSTANCE.lock().unwrap();
	// 	app.init_data();
	// 	hinst = app.hinst;
	// 	width = app.coord.width();
	// 	height = app.coord.height();
	// };
	// INSTANCE2.with(|app_ref| {
	// 	let mut app = app_ref.borrow_mut();
	// 	app.init();
	// 	app.msg_loop();
	// });

	App::msg_loop();
}


struct App {
	hinst: HINSTANCE,
	// hWnd
	hwnd_root: HWND,
	// WindowClass
//	wnd_proc: Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>,
	wndclass_atom: u16,
	coord: winrs_util::WndCoord,
	// ChildWindow:
	hwnd_edit: HWND,	// EditCtrl
	hwnd_btn: HWND,		// Button Ctrl
	// Font
	hfont: HFONT,
	font_face: winrs_util::WSTR,
	// Drag & Drop
	dd_buff: [u16;512],
	// Buffer: GETTEXT
	gettext_buff: [u16;2048],
}

impl App {

	pub fn new() -> Self {
		Self{
			hinst: HINSTANCE::default(),
			hwnd_root: HWND::default(),
			// WindowClass
//			wnd_proc: Some(wndproc),
			wndclass_atom: 0,
			// Window情報
			coord: winrs_util::WndCoord::new(0,0,0,0),
			// ChildWindow: EditCtrl
			hwnd_edit: HWND::default(),
			hwnd_btn: HWND::default(),
			// Font
			hfont: HFONT::default(),
			font_face: winrs_util::WSTR::new("Meiryo UI\0"),
			// Drag & Drop
			dd_buff: [0; 512],
			// Buffer: GETTEXT
			gettext_buff: [0; 2048],
		}
	}

	pub fn drop(&mut self) {
		unsafe {
			DeleteObject(self.hfont);
		}
	}

	pub fn init(&mut self) {
		unsafe {
			// hInstance作成
			self.hinst = GetModuleHandleW(PWSTR::NULL);
		}
		//
		self.make_font();
		self.make_wndclass();
		self.make_coord();
		self.init_window();
	}

	fn make_font(&mut self) {
		unsafe {
			self.hfont = CreateFontW(
				18,
				0,
				0,
				0,
				FW_NORMAL as i32,
				0,
				0,
				0,
				SHIFTJIS_CHARSET,
				OUT_TT_PRECIS,
				CLIP_DEFAULT_PRECIS,
				DEFAULT_QUALITY,
				FF_MODERN,
				self.font_face.as_pwstr(),
			);
		}
	}

	fn make_wndclass(&mut self) {
		unsafe {
			// WindowClass
			let mut wc = WNDCLASSW::default();
			wc.hCursor = LoadCursorW(HINSTANCE(0), IDC_ARROW);
			wc.hInstance = self.hinst;
			wc.lpszClassName = resource::CLASS_NAME.get().unwrap().as_pwstr();
			let style = CS_HREDRAW | CS_VREDRAW;
			wc.style = style;
			wc.lpfnWndProc = Some(wndproc);
			let brush: HGDIOBJ = GetStockObject(DKGRAY_BRUSH);
			//wc.hbrBackground = *(&brush as *const HGDIOBJ as *const HBRUSH);
			//wc.hbrBackground = HBRUSH(GetStockObject(DKGRAY_BRUSH).0);
			wc.hbrBackground = HBRUSH(brush.0);
			self.wndclass_atom = RegisterClassW(&wc);
		}
	}

	fn make_coord(&mut self) {
		// 座標調整オブジェクト作成
		self.coord.update_root(0,0, 800, 200);
	}

	fn init_window(&mut self) {
		unsafe {
			// CreateWindow
			self.hwnd_root = CreateWindowExW(
				WS_EX_ACCEPTFILES,
				resource::CLASS_NAME.get().unwrap().as_pwstr(),
				resource::WND_TITLE.get().unwrap().as_pwstr(),
				WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN,
				CW_USEDEFAULT,
				CW_USEDEFAULT,
				self.coord.width(),
				self.coord.height(),
				HWND(0),
				HMENU(0),
				self.hinst,
				std::ptr::null_mut(),
			);
			if self.hwnd_root == HWND::default() {
				println!("NG!");
			}
		}
	}

	pub fn msg_loop() {
		unsafe {
			// Message Loop
			let mut message = MSG::default();
			// while message.message != WM_QUIT {
			// 	if PeekMessageW(&mut message, HWND(0), 0, 0, PM_REMOVE).into() {
			// 		TranslateMessage(&mut message);
			// 		DispatchMessageW(&mut message);
			// 	}
			// }
			// if message.message == WM_QUIT {
			// 	println!("WM_QUIT");
			// 	PostQuitMessage(0);
			// }
			while GetMessageW(&mut message, HWND(0), 0, 0).into() {
				TranslateMessage(&mut message);
				DispatchMessageW(&mut message);
			}
		}
	}

	pub fn wndproc(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
		unsafe {
			match message {
				WM_CREATE => {
					self.on_create(window, message, wparam, lparam)
				}
				WM_PAINT => {
					println!("WM_PAINT");
					ValidateRect(window, std::ptr::null());
					LRESULT(0)
				}
				WM_SIZE => {
					self.on_size(window, message, wparam, lparam)
				}
				WM_DROPFILES => {
					self.on_drop_files(window, message, wparam, lparam)
				}
				WM_COMMAND => {
					self.on_command(window, message, wparam, lparam)
				}
				WM_DESTROY => {
					println!("WM_DESTROY");
					self.drop();
					PostQuitMessage(0);
					LRESULT(0)
				}
				_ => DefWindowProcW(window, message, wparam, lparam),
			}
		}
	}

	fn on_create(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
		unsafe {

			//let cs: CREATESTRUCTW = CREATESTRUCTW::into::<LPARAM>(lparam);
			let cs = lparam.0 as *const CREATESTRUCTW;

			// コモンコントロール初期化
			let icc = INITCOMMONCONTROLSEX::default();
			InitCommonControlsEx(&icc);

			// Edit Control 作成
			let mut rect: RECT = RECT::default();
			let res = GetClientRect(window, &mut rect );
			self.coord.update_root(rect.left, rect.top, rect.right, rect.bottom);
			self.coord.add_child(5, 30, 800, 200);
			let edit_coord = self.coord.get_child(0);
			//
			let mut ws_val = 0;
			ws_val |= (WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL | WS_CLIPSIBLINGS).0;
			ws_val |= (ES_AUTOHSCROLL | ES_MULTILINE) as u32;
			let ws = WINDOW_STYLE(ws_val);
			self.hwnd_edit = CreateWindowExW(
				WS_EX_ACCEPTFILES,
				resource::EDIT_CLASS_NAME.get().unwrap().as_pwstr(),
				resource::EDIT_DEF_STR.get().unwrap().as_pwstr(),
				ws,
				edit_coord.x(),
				edit_coord.y(),
				edit_coord.width(),
				edit_coord.height(),
				window,
				HMENU(0),
				(*cs).hInstance,
				std::ptr::null_mut(),
			);
			// EditControlにフォント指定
			SendMessageW(self.hwnd_edit, WM_SETFONT, WPARAM(self.hfont.0 as usize), LPARAM(1));
			// サブクラス化
			let ret = SetWindowSubclass(self.hwnd_edit, Some(wndproc_edit), 0, 0);

			// Button Control 作成 
			self.hwnd_btn = CreateWindowExW(
				WS_EX_ACCEPTFILES,
				resource::BUTTON_CLASS_NAME.get().unwrap().as_pwstr(),
				resource::BUTTON_DISP.get().unwrap().as_pwstr(),
				WS_CHILD | WS_VISIBLE,
				5,
				5,
				200,
				20,
				window,
				HMENU(1),					// 子ウィンドウのIDになる
				(*cs).hInstance,
				std::ptr::null_mut(),
			);
			// EditControlにフォント指定
			SendMessageW(self.hwnd_btn, WM_SETFONT, WPARAM(self.hfont.0 as usize), LPARAM(1));
		}

		LRESULT(0)
	}

	fn on_size(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
		unsafe {
			let mut rect: RECT = RECT::default();
			let res = GetClientRect(window, &mut rect );
			self.coord.update_root(rect.left, rect.top, rect.right, rect.bottom);
			self.coord.update_child(0);
			let edit_coord = self.coord.get_child(0);
			let result = MoveWindow(
				self.hwnd_edit,
				edit_coord.x(),
				edit_coord.y(),
				edit_coord.width(),
				edit_coord.height(),
				true
			);
		}

		LRESULT(0)
	}

	fn on_drop_files(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
		println!("WM_DROPFILES");

		unsafe {
			// D&Dされたファイル数を取得
			let hdrop = HDROP(wparam.0 as isize);
			let file_num = DragQueryFileW(hdrop, 0xFFFFFFFF, PWSTR(std::ptr::null_mut()), 0);
			// ファイルパス取得
			let mut dd_len: usize = 1;
			let mut dd_path :Vec<Vec<u16>> = Vec::with_capacity(file_num as usize);
			for i in 0..file_num {
				// パス取得
				let path_len = DragQueryFileW(hdrop, i, PWSTR(self.dd_buff.as_mut_ptr()), self.dd_buff.len() as u32);
				// バッファにコピー
				dd_path.push(self.dd_buff[0..path_len as usize].to_vec());
				// コピーしたバイト数をカウント
				dd_len += path_len as usize;
			}
			// D&D 処理終了
			DragFinish(hdrop);

			// EditCtrlにテキストを送信
			// 改行コード分を加算
			dd_len += ((file_num - 1) * 2) as usize;
			// 送信データを作成
			let mut send: Vec<u16> = Vec::with_capacity(dd_len);
			send.append(&mut dd_path[0]);
			for i in 1..file_num {
				// 改行追加
				send.push(*resource::UTF16_CR.get().unwrap());
				send.push(*resource::UTF16_LF.get().unwrap());
				// パス追加
				send.append(&mut dd_path[i as usize]);
			}
			// 末尾にNULL文字追加
			send.push(*resource::UTF16_NULL.get().unwrap());
			// EditCtrl内テキスト全体の長さを取得
			let whole_len = SendMessageW(self.hwnd_edit, WM_GETTEXTLENGTH, WPARAM(0),LPARAM(0));
			if whole_len.0 > 0 {
				// カーソルを一番最後に配置
				SendMessageW(self.hwnd_edit, EM_SETSEL, WPARAM(whole_len.0 as usize),LPARAM(whole_len.0 as isize));
				// 改行挿入
				SendMessageW(self.hwnd_edit, EM_REPLACESEL, WPARAM(0xFFFFFFFF),LPARAM(UTF16_CRLF.get().unwrap().as_pwstr().0 as isize));
			} else {
				// テキストが無ければ何もしない
			}
			// パスを追加
			SendMessageW(self.hwnd_edit, EM_REPLACESEL, WPARAM(0xFFFFFFFF),LPARAM(send.as_ptr() as isize));
		}

		LRESULT(0)
	}

	fn on_command(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
		// println!("WM_COMMAND");
		// println!("WPARAM size: {}", std::mem::size_of_val(&wparam)); -> 8 byte

		unsafe {
			let child_hwnd = HWND(lparam.0);
			let (notify, child_id) = winrs_util::wparam2command_button(wparam);

			if child_hwnd == self.hwnd_btn {
				// バッファ末尾を必ず\0にするためにバッファイサイズは-1して渡す
				let text_len = winrs_util::edit_get_text(self.hwnd_edit, self.gettext_buff.as_mut_ptr(), 2048-1);
				winrs_util::set_clipboard(self.gettext_buff.as_mut_ptr(), text_len);
				LRESULT(0)
			} else if child_hwnd == self.hwnd_edit {
				DefSubclassProc(window, message, wparam, lparam)
			} else {
				LRESULT(0)
			}
		}
	}


	pub fn wndproc_edit(&mut self, window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM, uidsubclass: usize, dwrefdata: usize) -> LRESULT {
		unsafe {
			match message {
				WM_DROPFILES => {
					self.on_drop_files(window, message, wparam, lparam);
				}
				_ => ()
			}
			DefSubclassProc(window, message, wparam, lparam)
		}
	}

}




extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
	//INSTANCE.lock().unwrap().wndproc(window, message, wparam, lparam)
	unsafe {
		INSTANCE.get_mut().unwrap().wndproc(window, message, wparam, lparam)
	}
	// INSTANCE2.with(|app_ref| -> LRESULT {
	// 	let mut app = app_ref.borrow_mut();
	// 	return app.wndproc(window, message, wparam, lparam)
	// })
}

extern "system" fn wndproc_edit(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM, uidsubclass: usize, dwrefdata: usize) -> LRESULT {
	//INSTANCE.lock().unwrap().wndproc(window, message, wparam, lparam)
	unsafe {
		INSTANCE.get_mut().unwrap().wndproc_edit(window, message, wparam, lparam, uidsubclass, dwrefdata)
	}
	// INSTANCE2.with(|app_ref| -> LRESULT {
	// 	let mut app = app_ref.borrow_mut();
	// 	return app.wndproc(window, message, wparam, lparam)
	// })
}
