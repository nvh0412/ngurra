use gpui::{div, prelude::*, Render, View, WindowContext};

use self::tab_bar::TabBar;

mod tab_bar;

pub struct TabBarContainer;

impl TabBarContainer {
    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self)
    }
}

impl Render for TabBarContainer {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div().w_full().flex().justify_center().child(TabBar::new("tab_bar"))
    }
}
