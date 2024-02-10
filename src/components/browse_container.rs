use gpui::{div, prelude::*, Render, View, WindowContext};

pub struct BrowseContainer;

impl BrowseContainer {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for BrowseContainer {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .mt_20()
            .flex()
            .justify_center()
            .child("Browse Card")
    }
}
