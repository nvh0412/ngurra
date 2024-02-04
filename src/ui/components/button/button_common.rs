use gpui::{AnyView, ElementId, WindowContext};

use crate::ui::{clickable::Clickable, disableable::Disableable};

pub trait ButtonCommon: Clickable + Disableable {
    fn id(&self) -> &ElementId;

    fn tooltip(self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self;
}
