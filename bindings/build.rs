fn main() {
	// https://github.com/microsoft/windows-rs
	// https://microsoft.github.io/windows-docs-rs/doc/bindings/Windows/
	// https://crates.io/crates/windows
	windows::build!(
		Windows::Win32::UI::WindowsAndMessaging::{
			GetMessageW, PeekMessageW, SendMessageW, 
			CreateWindowExW, DefWindowProcW, DispatchMessageW, PostQuitMessage,
			RegisterClassW, MessageBoxW, TranslateMessage,
			GetClientRect, MoveWindow, 
			MSG, WNDCLASSW, HMENU, CREATESTRUCTW, 
			CW_USEDEFAULT,
			WM_DESTROY, WM_PAINT, WM_CREATE, WM_QUIT, WM_SIZE, WM_SETFONT, 
			LoadCursorW, IDC_ARROW,
			WINDOW_STYLE, WINDOW_EX_STYLE,
			ES_AUTOHSCROLL, ES_MULTILINE,
			WNDCLASS_STYLES,
			MESSAGEBOX_STYLE,			// include: MB_OK
			PEEK_MESSAGE_REMOVE_TYPE,
		},
		Windows::Win32::UI::Controls::{
			InitCommonControlsEx,
			INITCOMMONCONTROLSEX, INITCOMMONCONTROLSEX_ICC
		},
		Windows::Win32::System::LibraryLoader::{
			GetModuleHandleW
		},
		Windows::Win32::Foundation::{
			HWND, LPARAM, WPARAM, HINSTANCE, LRESULT, PWSTR,
			RECT,
		},
		Windows::Win32::Graphics::Gdi::{
			ValidateRect, GetStockObject, UpdateWindow,
			HBRUSH, GET_STOCK_OBJECT_FLAGS,
			HGDIOBJ,
			HFONT, CreateFontW, FW_NORMAL, SHIFTJIS_CHARSET, FONT_OUTPUT_PRECISION, FONT_CLIP_PRECISION, FONT_QUALITY, FONT_PITCH_AND_FAMILY, 
		},
	);
}