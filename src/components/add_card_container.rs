use gpui::{div, prelude::*, Render, View, WindowContext};

pub struct AddCardContainer;

impl AddCardContainer {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for AddCardContainer {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .mt_20()
            .flex()
            .justify_center()
            .child("Add Card")
    }
}
