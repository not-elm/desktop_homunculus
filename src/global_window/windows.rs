use std::{char::{decode_utf16, REPLACEMENT_CHARACTER}, sync::Mutex};

use bevy::math::Rect;
use windows::{core::BOOL, Win32::{Foundation::{HWND, LPARAM}, UI::WindowsAndMessaging::{EnumWindows, GetForegroundWindow, GetWindowRect, GetWindowTextW, IsWindowVisible}}};

use super::{GlobalWindow, GlobalWindows};

static FOUND_WINDOWS: Mutex<Vec<GlobalWindow>> = Mutex::new(Vec::new());

extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL{
    let mut rect = windows::Win32::Foundation::RECT::default();
    unsafe{
        if IsWindowVisible(hwnd).into() && GetWindowRect(hwnd, &mut rect).is_ok() {
            let Ok(mut foud_windows) = FOUND_WINDOWS.lock() else{
                return BOOL(1);
            };
            let frame = Rect::new(
                rect.left as f32, 
                rect.bottom as f32, 
                rect.right as f32,
                 rect.top as f32,
            );
            let current = GetForegroundWindow();
            if 0. < frame.width() && 0. < frame.height() && current != hwnd{
                let mut buf = vec![0u16; 1024];
                 let len = GetWindowTextW(hwnd, &mut buf) as usize;
                let title = (0 < len).then(||{
                    decode_title(&buf, len)
                });
                println!("TITLE: {title:?}");
                foud_windows.push(GlobalWindow{
                    title,
                    frame,
                });
            }
        }
    }
    BOOL(1)
}


fn decode_title(buf: &[u16], len: usize) -> String {
    decode_utf16(buf[..len].iter().copied())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

pub fn obtain_global_windows() -> Option<GlobalWindows> {
    unsafe{
        EnumWindows(Some(enum_windows_proc), LPARAM(0)).ok()?;
    }
    FOUND_WINDOWS
        .lock()
        .map(|mut windows|{
            GlobalWindows::new(std::mem::take(&mut windows))
        })
        .ok()
}