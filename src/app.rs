
use bindings::{
	Windows::Win32::Foundation::{
		HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR,
		RECT,
	},
	Windows::Win32::UI::WindowsAndMessaging::{
		GetMessageW, PeekMessageW,
		CreateWindowExW, DefWindowProcW, DispatchMessageW, PostQuitMessage,
		RegisterClassW, MessageBoxW, TranslateMessage, 
		GetClientRect, MoveWindow, 
		MSG, WNDCLASSW, HMENU, CREATESTRUCTW, CW_USEDEFAULT,
		// Apis
		WM_DESTROY, WM_PAINT, WM_CREATE, WM_QUIT, WM_SIZE, 
		WINDOW_STYLE,
		WS_OVERLAPPEDWINDOW, WS_VISIBLE, WS_CHILDWINDOW, WS_CHILD, WS_BORDER, WS_VSCROLL,
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
	},
	Windows::Win32::UI::Controls::{
		InitCommonControlsEx,
		INITCOMMONCONTROLSEX, INITCOMMONCONTROLSEX_ICC
	},
	Windows::Win32::System::LibraryLoader::{
		GetModuleHandleW
	},
	Windows::Win32::Graphics::Gdi::{
		ValidateRect, GetStockObject, UpdateWindow,
		HBRUSH, GET_STOCK_OBJECT_FLAGS,
		DKGRAY_BRUSH,
		HGDIOBJ,
	},
};
use once_cell::sync::OnceCell;

use crate::winrs_util;
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

	pub fn init() {
		CLASS_NAME.set(winrs_util::WSTR::new("MyWindowClass\0"));
		WND_TITLE.set(winrs_util::WSTR::new("Win32 App\0"));
		EDIT_CLASS_NAME.set(winrs_util::WSTR::new("EDIT\0"));
		EDIT_DEF_STR.set(winrs_util::WSTR::new("初期テキスト\0"));
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
	// ChildWindow: EditCtrl
	hwnd_edit: HWND,
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
		}
	}

	pub fn init(&mut self) {
		unsafe {
			// hInstance作成
			self.hinst = GetModuleHandleW(PWSTR::NULL);
		}
		//
		self.make_wndclass();
		self.make_coord();
		self.init_window();
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
				WM_DESTROY => {
					println!("WM_DESTROY");
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
