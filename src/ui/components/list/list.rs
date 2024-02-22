use gpui::{div, AnyElement, SharedString, WindowContext};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub struct List {
    /// Message to display when the list is empty
    /// Defaults to "No items"
    empty_message: SharedString,
    toggle: Option<bool>,
    children: SmallVec<[AnyElement; 2]>,
}

impl List {
    pub fn new() -> Self {
        Self {
            empty_message: "No items".into(),
            toggle: None,
            children: SmallVec::new(),
        }
    }

    pub fn empty_message(mut self, empty_message: impl Into<SharedString>) -> Self {
        self.empty_message = empty_message.into();
        self
    }

    pub fn toggle(mut self, toggle: impl Into<Option<bool>>) -> Self {
        self.toggle = toggle.into();
        self
    }
}

impl ParentElement for List {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for List {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().flex().flex_col().w_full().py_1().map(|this| {
            match (self.children.is_empty(), self.toggle) {
                (false, _) => this.children(self.children),
                (true, Some(false)) => this,
                (true, _) => this.child(self.empty_message.clone()),
            }
        })
    }
}
