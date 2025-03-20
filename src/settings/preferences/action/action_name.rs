use crate::new_type;

new_type!(ActionName, String);

impl ActionName {
    pub const IDLE: &'static str = "idle";
    pub const SIT_DOWN: &'static str = "sit_down";
    pub const SITTING: &'static str = "sitting";
    pub const DRAG: &'static str = "drag";
    pub const DRAG_START: &'static str = "drag_start";
    pub const DROP: &'static str = "drag_drop";

    pub fn idle() -> Self {
        Self::from(Self::IDLE)
    }

    pub fn sit_down() -> Self {
        Self::from(Self::SIT_DOWN)
    }

    pub fn sitting() -> Self {
        Self::from(Self::SITTING)
    }

    pub fn drag_start() -> ActionName {
        ActionName::from(Self::DRAG_START)
    }

    pub fn drag() -> Self {
        Self::from(Self::DRAG)
    }

    pub fn drop() -> Self {
        Self::from(Self::DROP)
    }

    #[inline]
    pub fn is_index(&self) -> bool {
        self.0 == Self::IDLE
    }

    #[inline]
    pub fn is_sit_down(&self) -> bool {
        self.0 == Self::SIT_DOWN
    }

    #[inline]
    pub fn is_sitting(&self) -> bool {
        self.0 == Self::SITTING
    }

    #[inline]
    pub fn is_drag_start(&self) -> bool {
        self.0 == Self::DRAG_START
    }
}

impl Default for ActionName {
    fn default() -> Self {
        Self::idle()
    }
}
