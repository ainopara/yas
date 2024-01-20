use anyhow::Result;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::io::{stdin, Write};
use std::iter::once;
use std::mem::transmute;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::process;
use std::ptr::null_mut;
use std::{thread, time};

use log::{error, info, warn};
#[cfg(windows)]
use winapi::shared::windef::{HWND, POINT as WinPoint, RECT as WinRect};
#[cfg(windows)]
use winapi::um::winuser::{
    ClientToScreen, FindWindowW, GetAsyncKeyState, GetClientRect, SetForegroundWindow,
    SetProcessDPIAware, ShowWindow, SW_RESTORE, VK_F12, VK_RBUTTON,
};

use crate::common::PixelRect;
#[cfg(windows)]
use winapi::shared::minwindef::BOOL;
#[cfg(windows)]
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA};
#[cfg(windows)]
use winapi::um::securitybaseapi::{AllocateAndInitializeSid, CheckTokenMembership, FreeSid};
#[cfg(windows)]
use winapi::um::winnt::{
    DOMAIN_ALIAS_RID_ADMINS, PSID, SECURITY_BUILTIN_DOMAIN_RID, SECURITY_NT_AUTHORITY,
    SID_IDENTIFIER_AUTHORITY,
};

#[cfg(windows)]
pub fn encode_wide(s: String) -> Vec<u16> {
    let wide: Vec<u16> = OsStr::new(&s).encode_wide().chain(once(0)).collect();
    wide
}

#[cfg(windows)]
fn find_window(p_class_name: *const u16, p_window_name: *const u16) -> Result<HWND, String> {
    let result: HWND = unsafe { FindWindowW(p_class_name, p_window_name) };
    if result.is_null() {
        Err(String::from("cannot find window"))
    } else {
        Ok(result)
    }
}

#[cfg(windows)]
pub fn find_ys_window() -> Result<HWND, String> {
    let class_name = encode_wide(String::from("UnityWndClass"));
    let window_name = encode_wide(String::from("原神"));
    find_window(class_name.as_ptr(), window_name.as_ptr())
}

#[cfg(windows)]
pub fn find_window_by_name(name: &str) -> Result<HWND, String> {
    let window_name = encode_wide(String::from(name));
    find_window(null_mut(), window_name.as_ptr())
}

#[cfg(windows)]
unsafe fn get_client_rect_unsafe(hwnd: HWND) -> Result<PixelRect> {
    let mut rect: WinRect = WinRect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    GetClientRect(hwnd, &mut rect);
    let width: i32 = rect.right;
    let height: i32 = rect.bottom;

    let mut point: WinPoint = WinPoint { x: 0, y: 0 };
    ClientToScreen(hwnd, &mut point as *mut WinPoint);
    let left: i32 = point.x;
    let top: i32 = point.y;

    Ok(PixelRect {
        left,
        top,
        width,
        height,
    })
}

#[cfg(windows)]
pub fn get_client_rect(hwnd: HWND) -> Result<PixelRect> {
    unsafe { get_client_rect_unsafe(hwnd) }
}

pub fn sleep(ms: u32) {
    let time = time::Duration::from_millis(ms as u64);
    thread::sleep(time);
}

pub fn read_file_to_string(path: String) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn error_and_quit(msg: &str) -> ! {
    error!("{}, 按Enter退出", msg);
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Readline error");
    process::exit(0);
}

#[cfg(windows)]
unsafe fn is_admin_unsafe() -> bool {
    let mut authority: SID_IDENTIFIER_AUTHORITY = SID_IDENTIFIER_AUTHORITY {
        Value: SECURITY_NT_AUTHORITY,
    };
    let mut group: PSID = null_mut();
    let mut b = AllocateAndInitializeSid(
        &mut authority as *mut SID_IDENTIFIER_AUTHORITY,
        2,
        SECURITY_BUILTIN_DOMAIN_RID,
        DOMAIN_ALIAS_RID_ADMINS,
        0,
        0,
        0,
        0,
        0,
        0,
        &mut group as *mut PSID,
    );
    if b != 0 {
        let r = CheckTokenMembership(null_mut(), group, &mut b as *mut BOOL);
        if r == 0 {
            b = 0;
        }
        FreeSid(group);
    }

    b != 0
}

#[cfg(windows)]
pub fn is_admin() -> bool {
    unsafe { is_admin_unsafe() }
}

#[cfg(not(windows))]
pub fn is_admin() -> bool {
    true
}

#[cfg(windows)]
pub fn is_rmb_down() -> bool {
    unsafe {
        let state = GetAsyncKeyState(VK_RBUTTON);
        if state == 0 {
            return false;
        }

        state & 1 > 0
    }
}

#[cfg(windows)]
pub fn is_f12_down() -> bool {
    unsafe {
        let state = GetAsyncKeyState(VK_F12);
        if state == 0 {
            return false;
        }

        state & 1 > 0
    }
}

pub fn encode_lpcstr(s: &str) -> Vec<i8> {
    let mut arr: Vec<i8> = s.bytes().map(|x| x as i8).collect();
    arr.push(0);
    arr
}

#[cfg(windows)]
pub fn set_dpi_awareness() {
    // let os = os_info::get();

    let h_lib = unsafe {
        // let names = ["SHCore.dll"]

        LoadLibraryA(encode_lpcstr("Shcore.dll").as_ptr())
    };
    if h_lib.is_null() {
        info!("`Shcore.dll` not found");
        unsafe {
            SetProcessDPIAware();
        }
    } else {
        info!("`Shcore.dll` found");
        unsafe {
            let addr = GetProcAddress(h_lib, encode_lpcstr("SetProcessDpiAwareness").as_ptr());
            if addr.is_null() {
                warn!("cannot find process `SetProcessDpiAwareness`, but `Shcore.dll` exists");
                SetProcessDPIAware();
            } else {
                // func(PROCESS_DPI_AWARENESS) -> HRESULT
                let func = transmute::<*const (), fn(u32) -> i32>(addr as *const ());
                // SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
                func(2);
            }

            FreeLibrary(h_lib);
        }
    }

    // if os.version() >= &os_info::Version::from_string("8.1") {
    //     info!("Windows version >= 8.1");
    //     unsafe {
    //         SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
    //     }
    // } else {
    //     info!("Windows version < 8.1");
    //     unsafe {
    //         SetProcessDPIAware();
    //     }
    // }
}

#[cfg(windows)]
pub fn show_window_and_set_foreground(hwnd: HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
        SetForegroundWindow(hwnd);
    }
}

pub fn dump_json(data: &impl Serialize, path: PathBuf) -> Result<()> {
    let mut file = fs::File::create(path)?;
    let s = serde_json::to_string(data)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
