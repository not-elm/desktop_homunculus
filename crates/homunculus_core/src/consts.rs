pub struct CameraOrders;

impl CameraOrders {
    pub const EFFECT: isize = 0;
    pub const DEFAULT: isize = 1;
    pub const UI: isize = 2;
}

pub mod prelude {
    pub use crate::consts::CameraOrders;
}
