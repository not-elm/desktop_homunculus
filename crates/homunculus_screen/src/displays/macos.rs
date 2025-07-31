use crate::displays::{DisplayId, GlobalDisplay};
use bevy::math::{Rect, Vec2};
use cocoa::appkit::NSScreen;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use core_graphics_helmer_fork::display::CGDirectDisplayID;
use objc::{msg_send, sel, sel_impl};
use screencapturekit::sc_shareable_content::SCShareableContent;

pub fn all_displays() -> Vec<GlobalDisplay> {
    let content = SCShareableContent::current();
    content
        .displays
        .into_iter()
        .map(|display| {
            let x = display.frame.origin.x as f32;
            let y = display.frame.origin.y as f32;
            let xo = x + display.frame.size.width as f32;
            let yo = y + display.frame.size.height as f32;
            GlobalDisplay {
                id: DisplayId(display.display_id),
                title: get_display_name(display.display_id),
                frame: Rect::from_corners(Vec2::new(x, y), Vec2::new(xo, yo)),
            }
        })
        .collect()
}

#[allow(unexpected_cfgs)]
fn get_display_name(display_id: CGDirectDisplayID) -> String {
    unsafe {
        // Get all screens
        let screens: id = NSScreen::screens(nil);
        let count: u64 = msg_send![screens, count];

        for i in 0..count {
            let screen: id = msg_send![screens, objectAtIndex: i];
            let device_description: id = msg_send![screen, deviceDescription];
            let display_id_number: id = msg_send![device_description, objectForKey: NSString::alloc(nil).init_str("NSScreenNumber")];
            let display_id_number: u32 = msg_send![display_id_number, unsignedIntValue];

            if display_id_number == display_id {
                let localized_name: id = msg_send![screen, localizedName];
                let name: *const i8 = msg_send![localized_name, UTF8String];
                return std::ffi::CStr::from_ptr(name)
                    .to_string_lossy()
                    .into_owned();
            }
        }

        format!("Unknown Display {display_id}")
    }
}
