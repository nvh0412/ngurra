use gpui::*;

use crate::components::tab_bar_container::TabBarContainer;

actions!(
    ngurra,
    [
        Hide
    ]
);

pub struct Ngurra {
    view: AnyView
}

impl Ngurra {
    pub fn new(view: AnyView) -> Self {
        Self { view }
    }
}

impl Render for Ngurra {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(TabBarContainer::view(cx))
            .child(self.view.clone())
    }
}

pub fn init(cx: &mut AppContext) {
    cx.on_action(|_: &Hide, cx| cx.hide() )
}
