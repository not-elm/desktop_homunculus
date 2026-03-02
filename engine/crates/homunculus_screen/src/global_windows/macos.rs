use crate::global_windows::{GlobalWindow, GlobalWindows};
use bevy::math::{Rect, Vec2};
use core_foundation::array::CFArray;
use core_foundation::base::{TCFType, TCFTypeRef, ToVoid};
use core_foundation::dictionary::CFDictionary;
use core_foundation::number::{__CFBoolean, CFBooleanGetValue, CFNumber};
use core_foundation::string::CFString;
use core_graphics::window::kCGNullWindowID;
use core_graphics::window::{CGWindowListCopyWindowInfo, kCGWindowListOptionOnScreenOnly};
use objc2::MainThreadMarker;
use objc2::ffi::NSInteger;
use objc2_app_kit::NSApp;
use std::ffi::c_void;
use std::sync::OnceLock;

/// Wrapper to allow `CFString` in `OnceLock` statics.
///
/// SAFETY: `CFString` wraps an immutable, reference-counted Core Foundation string.
/// Core Foundation string objects are thread-safe for concurrent read access.
struct SyncCFString(CFString);
unsafe impl Send for SyncCFString {}
unsafe impl Sync for SyncCFString {}

macro_rules! cf_key {
    ($fn_name:ident, $value:expr) => {
        fn $fn_name() -> &'static CFString {
            static KEY: OnceLock<SyncCFString> = OnceLock::new();
            &KEY.get_or_init(|| SyncCFString(CFString::new($value))).0
        }
    };
}

cf_key!(key_window_owner_name, "kCGWindowOwnerName");
cf_key!(key_window_is_onscreen, "kCGWindowIsOnscreen");
cf_key!(key_window_bounds, "kCGWindowBounds");
cf_key!(key_window_layer, "kCGWindowLayer");
cf_key!(key_window_number, "kCGWindowNumber");
cf_key!(key_x, "X");
cf_key!(key_y, "Y");
cf_key!(key_width, "Width");
cf_key!(key_height, "Height");

/// # Safety
///
/// Just in case, it is better to read this on the MainThread?
pub unsafe fn app_window_numbers() -> Vec<u64> {
    let Some(mtm) = MainThreadMarker::new() else {
        return Vec::new();
    };
    NSApp(mtm)
        .windows()
        .iter()
        .map(|w| w.windowNumber() as u64)
        .collect()
}

pub fn find_all() -> Option<GlobalWindows> {
    let windows = obtain_all_windows()?;
    let mut areas = windows
        .iter()
        .filter(|window| unsafe { is_visible_window(window) })
        .flat_map(|window_dict| convert_to_sitting_area(&window_dict))
        .enumerate()
        .collect::<Vec<_>>();

    areas.sort_by_key(|(index, area)| area.window_layer + *index as i64);
    Some(GlobalWindows::new(
        areas.into_iter().map(|(_, area)| area).collect(),
    ))
}

/// # Safety
///
/// Just in case, it is better to read this on the MainThread?
pub unsafe fn find_above_global_window(window_frame: Rect) -> Option<GlobalWindow> {
    find_all()?
        .0
        .iter()
        .find(|w| !w.frame.intersect(window_frame).is_empty())
        .cloned()
}

pub fn find_window_from_number(target_window_num: NSInteger) -> Option<GlobalWindow> {
    let windows = obtain_all_windows()?;
    windows
        .iter()
        .flat_map(|dict| {
            let area = convert_to_sitting_area(&dict)?;
            if area.window_number == target_window_num {
                Some(area)
            } else {
                None
            }
        })
        .next()
}

fn convert_to_sitting_area(dict: &CFDictionary) -> Option<GlobalWindow> {
    let title = obtain_window_owner_name(dict);
    if title.as_ref().is_some_and(|title| title == "Window Server") {
        return None;
    }
    let window_layer = unsafe { obtain_window_layer(dict) };
    let window_number = unsafe { window_number(dict)? };

    Some(GlobalWindow {
        title,
        frame: unsafe { obtain_window_frame(dict)? },
        window_number,
        window_layer,
    })
}

fn obtain_all_windows() -> Option<CFArray<CFDictionary>> {
    let windows =
        unsafe { CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, kCGNullWindowID) };
    if windows.is_null() {
        return None;
    }
    let windows: CFArray<CFDictionary> = unsafe { CFArray::wrap_under_create_rule(windows) };
    Some(windows)
}

fn obtain_window_owner_name(window: &CFDictionary) -> Option<String> {
    window
        .find(key_window_owner_name().to_void())
        .map(|refs| unsafe { CFString::wrap_under_get_rule(refs.as_void_ptr() as *const _) })
        .map(|str| str.to_string())
}

unsafe fn is_visible_window(window_dict: &CFDictionary) -> bool {
    // Offscreen if non-zero?
    let visible = unsafe { find_as_cf_number(window_dict, key_window_layer()) }
        .and_then(|layer| layer.to_i64())
        .is_some_and(|layer| layer == 0);
    if !visible {
        return false;
    }

    is_on_screen(window_dict)
}

unsafe fn obtain_window_layer(window_dict: &CFDictionary) -> i64 {
    unsafe { find_as_cf_number(window_dict, key_window_layer()) }
        .and_then(|layer| layer.to_i64())
        .unwrap_or_default()
}

unsafe fn window_number(window_dict: &CFDictionary) -> Option<NSInteger> {
    let window_number = unsafe {
        find_as_cf_number(window_dict, key_window_number()).and_then(|number| number.to_i64())?
    };
    Some(window_number as NSInteger)
}

fn is_on_screen(window_dict: &CFDictionary) -> bool {
    window_dict
        .find(key_window_is_onscreen().to_void())
        .map(|v| v.cast::<__CFBoolean>())
        .map(|is_on_screen| unsafe { CFBooleanGetValue(is_on_screen) })
        .unwrap_or(false)
}

unsafe fn obtain_window_frame(window: &CFDictionary) -> Option<Rect> {
    let bounds_ref = window.find(key_window_bounds().to_void())?;
    let bounds_dict: CFDictionary<*const c_void, *const c_void> =
        unsafe { CFDictionary::wrap_under_get_rule(bounds_ref.cast()) };
    let find_num = |key: &CFString| unsafe { find_as_cf_number(&bounds_dict, key)?.to_f32() };
    let viewport_pos = Vec2::new(find_num(key_x())?, find_num(key_y())?);
    let size = Vec2::new(find_num(key_width())?, find_num(key_height())?);
    Some(Rect::from_center_size(viewport_pos + size / 2.0, size))
}

unsafe fn find_as_cf_number(dictionary: &CFDictionary, key: &CFString) -> Option<CFNumber> {
    let num_ref = dictionary.find(key.to_void())?;
    let number = unsafe { CFNumber::wrap_under_get_rule(num_ref.cast()) };
    Some(number)
}
