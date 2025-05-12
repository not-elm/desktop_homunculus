use crate::prelude::GlobalViewport;
use bevy::prelude::*;
use bevy_vrm1::vrm::VrmBone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnDragStartEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, DragStart)> for OnDragStartEvent {
    fn from((global_viewport, bone, _): (GlobalViewport, Option<VrmBone>, DragStart)) -> Self {
        Self {
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnDragEvent {
    pub delta: Vec2,
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Drag)> for OnDragEvent {
    fn from((global_viewport, bone, event): (GlobalViewport, Option<VrmBone>, Drag)) -> Self {
        Self {
            delta: event.delta,
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnDragEndEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, DragEnd)> for OnDragEndEvent {
    fn from((global_viewport, bone, _): (GlobalViewport, Option<VrmBone>, DragEnd)) -> Self {
        Self {
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Button {
    Primary,
    /// The secondary pointer button
    Secondary,
    /// The tertiary pointer button
    Middle,
}

impl From<PointerButton> for Button {
    fn from(button: PointerButton) -> Self {
        match button {
            PointerButton::Primary => Button::Primary,
            PointerButton::Secondary => Button::Secondary,
            PointerButton::Middle => Button::Middle,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerPressedEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub button: Button,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Pressed)> for OnPointerPressedEvent {
    fn from((global_viewport, bone, event): (GlobalViewport, Option<VrmBone>, Pressed)) -> Self {
        Self {
            global_viewport,
            button: event.button.into(),
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerMoveEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub delta: Vec2,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Move)> for OnPointerMoveEvent {
    fn from((global_viewport, bone, event): (GlobalViewport, Option<VrmBone>, Move)) -> Self {
        Self {
            global_viewport,
            delta: event.delta,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerReleasedEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub button: Button,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Released)> for OnPointerReleasedEvent {
    fn from((global_viewport, bone, event): (GlobalViewport, Option<VrmBone>, Released)) -> Self {
        Self {
            global_viewport,
            button: event.button.into(),
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerOverEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Over)> for OnPointerOverEvent {
    fn from((global_viewport, bone, _): (GlobalViewport, Option<VrmBone>, Over)) -> Self {
        Self {
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerOutEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Out)> for OnPointerOutEvent {
    fn from((global_viewport, bone, _): (GlobalViewport, Option<VrmBone>, Out)) -> Self {
        Self {
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnPointerCancelEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Cancel)> for OnPointerCancelEvent {
    fn from((global_viewport, bone, _): (GlobalViewport, Option<VrmBone>, Cancel)) -> Self {
        Self {
            global_viewport,
            bone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OnClickEvent {
    #[serde(rename = "globalViewport")]
    pub global_viewport: GlobalViewport,
    pub button: Button,
    pub bone: Option<VrmBone>,
}

impl From<(GlobalViewport, Option<VrmBone>, Click)> for OnClickEvent {
    fn from((global_viewport, bone, event): (GlobalViewport, Option<VrmBone>, Click)) -> Self {
        Self {
            global_viewport,
            button: event.button.into(),
            bone,
        }
    }
}
