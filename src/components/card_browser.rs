use gpui::{div, prelude::*, Render, View, WindowContext};

pub struct CardBrowserView;

impl CardBrowserView {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for CardBrowserView {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div().mt_20().flex().justify_center().child("Browse Card")
    }
}
