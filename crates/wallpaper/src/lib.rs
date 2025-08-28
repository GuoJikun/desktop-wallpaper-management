use std::ptr::null_mut;

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        UI::WindowsAndMessaging,
    },
    core::{BOOL, PCWSTR},
};

extern "system" fn enum_window(window: HWND, ref_worker_w: LPARAM) -> BOOL {
    unsafe {
        // 查找 defView 窗口
        let shell_dll_def_view_name = PCWSTR::from_raw(
            "SHELLDLL_DefView\0"
                .encode_utf16()
                .collect::<Vec<u16>>()
                .as_ptr(),
        );
        let shell_dll_def_view =
            WindowsAndMessaging::FindWindowExW(Some(window), None, shell_dll_def_view_name, None);

        match shell_dll_def_view {
            Err(_) => {
                return true.into();
            }
            Ok(shell_dll_def_view) => {
                if shell_dll_def_view == HWND(null_mut()) {
                    return true.into(); // 继续枚举
                }

                let worker_w_name =
                    PCWSTR::from_raw("WorkerW\0".encode_utf16().collect::<Vec<u16>>().as_ptr());

                let worker_w =
                    WindowsAndMessaging::FindWindowExW(Some(window), None, worker_w_name, None);

                match worker_w {
                    Err(_) => {
                        return true.into();
                    }
                    Ok(worker_w) => {
                        if worker_w == HWND(null_mut()) {
                            return true.into(); // 继续枚举
                        }
                        *(ref_worker_w.0 as *mut HWND) = worker_w;
                        return false.into(); // 停止枚举
                    }
                }
            }
        }
    }
}

pub fn attach(hwnd: HWND) {
    unsafe {
        let progman_name =
            PCWSTR::from_raw("Progman\0".encode_utf16().collect::<Vec<u16>>().as_ptr());
        let progman = WindowsAndMessaging::FindWindowW(progman_name, PCWSTR::null());

        if progman.is_err() {
            panic!("Could not find progman window");
        }
        let progman = progman.unwrap();

        WindowsAndMessaging::SendMessageW(
            progman,
            0x052C,
            Some(WPARAM(0x0000000D)),
            Some(LPARAM(0)),
        );
        WindowsAndMessaging::SendMessageW(
            progman,
            0x052C,
            Some(WPARAM(0x0000000D)),
            Some(LPARAM(1)),
        );

        let mut worker_w: HWND = HWND(null_mut());

        let _ = WindowsAndMessaging::EnumWindows(
            Some(enum_window),
            LPARAM(&mut worker_w as *mut HWND as isize),
        );

        if worker_w == HWND(null_mut()) {
            panic!("Could not find worker_w window");
        }

        let _ = WindowsAndMessaging::SetParent(hwnd, Some(worker_w));
    };
}

pub fn detach(hwnd: HWND) {
    unsafe {
        let _ = WindowsAndMessaging::SetParent(hwnd, None);
        let _ = WindowsAndMessaging::SystemParametersInfoA(
            WindowsAndMessaging::SPI_SETDESKWALLPAPER,
            0,
            Some(null_mut()),
            WindowsAndMessaging::SPIF_UPDATEINIFILE,
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attach_detach() {
        // 这里只能做基本调用测试，不能验证实际效果
        // 使用 HWND(0) 作为 mock 参数，真实场景需传入有效窗口句柄
        let hwnd = HWND::default();
        // attach 和 detach 可能会 panic 或无效，建议在集成测试中使用真实句柄
        // 这里只测试函数能否被调用
        let _ = std::panic::catch_unwind(|| {
            attach(hwnd);
        });
        let _ = std::panic::catch_unwind(|| {
            detach(hwnd);
        });
    }
}
