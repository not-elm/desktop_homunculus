use crate::displays::{DisplayId, GlobalDisplay};
use bevy::{
    math::{Rect, Vec2},
    utils::default,
};
use std::char::{REPLACEMENT_CHARACTER, decode_utf16};
use windows::{
    Win32::{
        Foundation::{LPARAM, RECT},
        Graphics::Gdi::{
            DISPLAY_DEVICEW, EnumDisplayDevicesW, EnumDisplayMonitors, GetMonitorInfoW, HDC,
            HMONITOR, MONITORINFO, MONITORINFOEXW,
        },
    },
    core::{BOOL, PCWSTR},
};

pub fn all_displays() -> Vec<GlobalDisplay> {
    let mut displays: Vec<GlobalDisplay> = Vec::new();
    let data = LPARAM(&mut displays as *mut Vec<GlobalDisplay> as isize);
    unsafe {
        let _ = EnumDisplayMonitors(None, None, Some(enum_monitor_proc), data);
    }
    displays
}

unsafe extern "system" fn enum_monitor_proc(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _rect: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let displays = unsafe { &mut *(lparam.0 as *mut Vec<GlobalDisplay>) };
    if let Some(display) = build_display(hmonitor, displays.len() as u32) {
        displays.push(display);
    }
    BOOL(1)
}

fn build_display(hmonitor: HMONITOR, index: u32) -> Option<GlobalDisplay> {
    let info = obtain_monitor_info(hmonitor)?;
    let rc = info.monitorInfo.rcMonitor;
    let device_name = decode_device_name(&info.szDevice);
    // パース失敗時は index + 1000 で正常IDとの衝突を回避
    let id = parse_display_id(&device_name).unwrap_or(index + 1000);
    let title = obtain_friendly_name(&device_name).unwrap_or(device_name);
    let frame = Rect::from_corners(
        Vec2::new(rc.left as f32, rc.top as f32),
        Vec2::new(rc.right as f32, rc.bottom as f32),
    );
    Some(GlobalDisplay {
        id: DisplayId(id),
        title,
        frame,
    })
}

fn obtain_monitor_info(hmonitor: HMONITOR) -> Option<MONITORINFOEXW> {
    let mut info = MONITORINFOEXW::default();
    info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    unsafe {
        GetMonitorInfoW(
            hmonitor,
            (&mut info as *mut MONITORINFOEXW).cast::<MONITORINFO>(),
        )
        .ok()
        .ok()?;
    }
    Some(info)
}

fn decode_device_name(sz_device: &[u16]) -> String {
    let len = sz_device
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(sz_device.len());
    decode_utf16(sz_device[..len].iter().copied())
        .map(|r| r.unwrap_or(REPLACEMENT_CHARACTER))
        .collect()
}

fn parse_display_id(device_name: &str) -> Option<u32> {
    device_name
        .strip_prefix("\\\\.\\DISPLAY")
        .and_then(|n| n.parse::<u32>().ok())
}

fn obtain_friendly_name(device_name: &str) -> Option<String> {
    let wide: Vec<u16> = device_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut device = DISPLAY_DEVICEW {
        cb: std::mem::size_of::<DISPLAY_DEVICEW>() as u32,
        ..default()
    };
    let success = unsafe { EnumDisplayDevicesW(PCWSTR(wide.as_ptr()), 0, &mut device, 0) };
    if !success.as_bool() {
        return None;
    }
    let name = decode_device_name(&device.DeviceString);
    (!name.is_empty()).then_some(name)
}
