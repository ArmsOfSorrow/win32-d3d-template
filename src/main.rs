extern crate winapi;
extern crate wio;

use game::Game;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::windef::{HBRUSH, HMENU, HWND};
use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize, COINITBASE_MULTITHREADED};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, LoadIconW, PeekMessageW,
    PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage, COLOR_WINDOW, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, PM_REMOVE, SW_SHOW, WM_ACTIVATEAPP, WM_DESTROY,
    WM_ENTERSIZEMOVE, WM_EXITSIZEMOVE, WM_GETMINMAXINFO, WM_MENUCHAR, WM_PAINT, WM_POWERBROADCAST,
    WM_QUIT, WM_SIZE, WM_SYSKEYDOWN, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

mod game;
mod step_timer;

//TODO: mark everything as unsafe

fn main() {
    unsafe {
        //TODO: XMVerifyCPUSupport is missing. There are no bindings since it's c++.

        let hr = CoInitializeEx(std::ptr::null_mut(), COINITBASE_MULTITHREADED);
        if failed(hr) {
            std::process::exit(1);
        }

        let game = Game::new();

        //https://stackoverflow.com/questions/1749972/determine-the-current-hinstance
        let hinstance = GetModuleHandleW(std::ptr::null_mut());

        let wndclass_name: Vec<u16> = OsStr::new("testclassname")
            .encode_wide()
            .chain(once(0))
            .collect();

        let idi_icon: Vec<u16> = OsStr::new("IDI_ICON")
            .encode_wide()
            .chain(once(0))
            .collect();

        //window class registration
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: LoadIconW(hinstance, idi_icon.as_ptr()),
            hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: wndclass_name.as_ptr(),
            hIconSm: LoadIconW(hinstance, idi_icon.as_ptr()),
        };

        let registered = RegisterClassExW(&wnd_class as *const WNDCLASSEXW) != 0;
        if !registered {
            std::process::exit(1);
        } else {
            let mut width = 0;
            let mut height = 0;

            game.get_default_size(&mut width, &mut height);

            let wnd_name: Vec<u16> = OsStr::new("window").encode_wide().chain(once(0)).collect();
            let hwnd = CreateWindowExW(
                0,
                wndclass_name.as_ptr(),
                wnd_name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                800,
                600,
                0 as HWND,
                0 as HMENU,
                0 as HINSTANCE,
                std::ptr::null_mut(),
            );

            if hwnd != std::ptr::null_mut() {
                ShowWindow(hwnd, SW_SHOW);
                // game.initialize(hwnd, width: i32, height: i32)
                let mut msg: MSG = std::mem::zeroed();
                while WM_QUIT != msg.message {
                    if PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    } else {
                        //game tick comes here
                    }
                }

                CoUninitialize();

                //not sure what to do about msg.wparam.
                //probably std::process::exit as in the other cases.
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    message: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match message {
        WM_PAINT => {}
        WM_SIZE => {}
        WM_ENTERSIZEMOVE => {}
        WM_EXITSIZEMOVE => {}
        WM_GETMINMAXINFO => {}
        WM_ACTIVATEAPP => {}
        WM_POWERBROADCAST => {}
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        WM_SYSKEYDOWN => {}
        WM_MENUCHAR => {}
        _ => {}
    };

    DefWindowProcW(hwnd, message, w_param, l_param)
}

fn failed(hr: HRESULT) -> bool {
    hr < 0
}
